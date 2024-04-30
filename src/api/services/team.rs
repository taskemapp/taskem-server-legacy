use std::str::FromStr;
use std::sync::Arc;

use autometrics::autometrics;
use derive_new::new;
use tonic::{async_trait, Request, Response, Status};
use tracing::info;
use uuid::Uuid;

use crate::domain::constants::MIDDLEWARE_AUTH_SESSION_KEY;
use crate::domain::models::team::team_information::TeamInformation;
use crate::domain::models::team::team_member::TeamMember;
use crate::domain::models::team::team_role::TeamRole;
use crate::domain::repositories::role::RoleRepository;
use crate::domain::repositories::team::TeamRepository;
use crate::extract_user_id_from_metadata;
use crate::team::team_server::Team;
use crate::team::{
    ChangeTeamRole, CreateTeamRequest, CreateTeamResponse, GetAllTeamsResponse, GetTeamRequest,
    GetTeamRolesRequest, GetTeamRolesResponse, JoinTeamRequest, JoinTeamResponse, LeaveTeamRequest,
    LeaveTeamResponse, Role, RolePermission, TeamResponse, UserInfo,
};

#[derive(new)]
pub struct TeamServiceImpl {
    pub(self) team_repository: Arc<dyn TeamRepository>,
    pub(self) role_repository: Arc<dyn RoleRepository>,
}

impl TeamServiceImpl {
    fn map_teams_to_response(&self, teams: Vec<TeamInformation>) -> Vec<TeamResponse> {
        let role_repository = self.role_repository.clone();
        teams
            .iter()
            .map(|team| TeamResponse {
                id: team.id.to_string(),
                name: team.name.clone(),
                description: team.description.clone(),
                members: team
                    .members
                    .iter()
                    .map(|member| {
                        let role = role_repository
                            .get_by_team_and_user_id(&team.id, &member.id)
                            .unwrap_or_else(|_| {
                                panic!("Failed to get role by user: {}", &member.id)
                            });

                        UserInfo {
                            id: member.id.to_string(),
                            user_name: member.user_name.clone(),
                            role: role.name,
                        }
                    })
                    .collect(),
                creator: team.creator.to_string(),
            })
            .collect()
    }
}

#[async_trait]
#[autometrics]
impl Team for TeamServiceImpl {
    async fn get(
        &self,
        request: Request<GetTeamRequest>,
    ) -> Result<Response<TeamResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let get_request = request.into_inner();

        let team_id = Uuid::from_str(get_request.team_id.as_str())
            .map_err(|_| Status::invalid_argument("Invalid team id"))?;

        match team_repository.get(&team_id) {
            Ok(team) => Ok(Response::new(TeamResponse {
                id: team.id.to_string(),
                name: team.name,
                description: team.description,
                members: team
                    .members
                    .iter()
                    .map(|member| {
                        let role = role_repository
                            .get_by_team_and_user_id(&team_id, &member.id)
                            .unwrap_or_else(|_| {
                                panic!("Failed to get role by user: {}", &member.id)
                            });

                        UserInfo {
                            id: member.id.to_string(),
                            user_name: member.user_name.clone(),
                            role: role.name,
                        }
                    })
                    .collect(),
                creator: team.creator.to_string(),
            })),
            Err(e) => Err(Status::internal(format!("Internal Server Error: {}", e))),
        }
    }

    async fn get_user_teams(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetAllTeamsResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let user_id = extract_user_id_from_metadata!(&request);

        let user_teams = team_repository
            .get_user_teams(&user_id)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        let teams: Vec<TeamResponse> = self.map_teams_to_response(user_teams);

        Ok(Response::new(GetAllTeamsResponse { teams }))
    }

    async fn get_all_can_join(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetAllTeamsResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let user_id = extract_user_id_from_metadata!(&request);

        let team_can_join = team_repository
            .get_all_can_join(&user_id)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        let teams: Vec<TeamResponse> = self.map_teams_to_response(team_can_join);

        Ok(Response::new(GetAllTeamsResponse { teams }))
    }

    async fn create(
        &self,
        request: Request<CreateTeamRequest>,
    ) -> Result<Response<CreateTeamResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let user_id = extract_user_id_from_metadata!(&request);

        let sign_up_request = request.into_inner();

        let new_team = &mut TeamInformation::from(sign_up_request);
        new_team.creator = user_id;

        let admin_role = TeamRole {
            id: Uuid::now_v7(),
            team_id: new_team.id,
            name: String::from("Admin"),
            priority: 0,
            can_add_task: true,
            can_assign_task: true,
            can_approve_task: true,
            can_invite_in_team: true,
            can_create_roles: true,
        };

        let manager_role = TeamRole {
            id: Uuid::now_v7(),
            team_id: new_team.id,
            name: String::from("Manager"),
            priority: 1,
            can_add_task: true,
            can_assign_task: true,
            can_approve_task: true,
            can_invite_in_team: true,
            can_create_roles: true,
        };

        let member_role = TeamRole {
            id: Uuid::now_v7(),
            team_id: new_team.id,
            name: String::from("Member"),
            priority: 2,
            can_add_task: false,
            can_assign_task: false,
            can_approve_task: false,
            can_invite_in_team: false,
            can_create_roles: false,
        };

        let created_team = team_repository
            .create(new_team)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        role_repository
            .create(&admin_role)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        let creator = &TeamMember {
            id: Uuid::now_v7(),
            user_id,
            team_id: created_team.id,
            role_id: admin_role.id,
        };

        team_repository
            .join(creator)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        role_repository
            .create(&manager_role)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        role_repository
            .create(&member_role)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        Ok(Response::new(CreateTeamResponse {
            message: String::from("Team successfully created"),
            team_id: created_team.id.to_string(),
        }))
    }

    async fn join(
        &self,
        request: Request<JoinTeamRequest>,
    ) -> Result<Response<JoinTeamResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let join_request = request.into_inner();

        let user_id = extract_user_id_from_metadata!(&request);

        let team_id = Uuid::from_str(join_request.team_id.as_str())
            .map_err(|_| Status::invalid_argument("Invalid team id"))?;

        let team_role = role_repository
            .get_lowest_priority(&team_id)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        let new_member = &TeamMember {
            id: Uuid::now_v7(),
            user_id,
            team_id,
            role_id: team_role.id,
        };

        team_repository
            .join(new_member)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        Ok(Response::new(JoinTeamResponse {
            message: String::from("Successfully joined to team"),
        }))
    }

    async fn get_roles(
        &self,
        request: Request<GetTeamRolesRequest>,
    ) -> Result<Response<GetTeamRolesResponse>, Status> {
        let role_request = request.into_inner();
        let role_repository = self.role_repository.clone();

        let team_id = Uuid::from_str(role_request.team_id.as_str())
            .map_err(|_| Status::invalid_argument("Invalid team id"))?;

        let team_roles = role_repository
            .get_all_for_team(&team_id)
            .map_err(|e| Status::internal(format!("Internal Server Error: {e}", )))?;

        Ok(Response::new(GetTeamRolesResponse {
            roles: team_roles
                .iter()
                .map(|role| Role {
                    role_name: role.name.clone(),
                    priority: role.priority,
                    permission: Some(RolePermission {
                        can_add_task: role.can_add_task,
                        can_assign_task: role.can_assign_task,
                        can_approve_task: role.can_approve_task,
                        can_invite_in_team: role.can_invite_in_team,
                        can_create_roles: role.can_create_roles,
                    }),
                })
                .collect(),
        }))
    }

    async fn change_role(
        &self,
        request: Request<ChangeTeamRole>,
    ) -> Result<Response<Role>, Status> {
        let role_repository = self.role_repository.clone();

        let called_user_id = extract_user_id_from_metadata!(&request);

        let change_role_request = request.into_inner();

        let user_id = Uuid::from_str(change_role_request.user_id.as_str())
            .map_err(|_| Status::invalid_argument("Invalid user id"))?;

        let role_id = Uuid::from_str(change_role_request.role_id.as_str())
            .map_err(|_| Status::invalid_argument("Invalid role id"))?;

        let updated_team_role = role_repository
            .update(&role_id, &user_id)
            .map_err(|e| Status::internal(format!("Internal Server Error: {}", e)))?;

        let called_role = match role_repository
            .get_by_team_and_user_id(&updated_team_role.team_id, &called_user_id)
        {
            Ok(value) => value,
            Err(e) => {
                return Err(Status::internal(format!("Internal Server Error: {}", e)));
            }
        };

        if !called_role.can_create_roles {
            return Err(Status::permission_denied(
                "You don' have a permission to do that".to_string(),
            ));
        }

        let role_permission = RolePermission {
            can_add_task: updated_team_role.can_add_task,
            can_assign_task: updated_team_role.can_assign_task,
            can_approve_task: updated_team_role.can_approve_task,
            can_invite_in_team: updated_team_role.can_invite_in_team,
            can_create_roles: updated_team_role.can_create_roles,
        };

        Ok(Response::new(Role {
            role_name: updated_team_role.name,
            priority: updated_team_role.priority,
            permission: Some(role_permission),
        }))
    }

    async fn leave(
        &self,
        request: Request<LeaveTeamRequest>,
    ) -> Result<Response<LeaveTeamResponse>, Status> {
        let _leave_request = request.into_inner();
        let _team_repository = self.team_repository.clone();

        todo!()
    }
}
