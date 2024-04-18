use crate::domain::constants::MIDDLEWARE_AUTH_SESSION_KEY;
use crate::domain::models::team::team_information::TeamInformation;
use crate::domain::models::team::team_member::TeamMember;
use crate::domain::models::team::team_role::TeamRole;
use crate::domain::repositories::role::RoleRepository;
use crate::domain::repositories::team::TeamRepository;
use crate::team::team_server::Team;
use crate::team::{
    ChangeTeamRole, CreateTeamRequest, CreateTeamResponse, GetAllTeamsResponse, GetTeamRequest,
    GetTeamRolesRequest, GetTeamRolesResponse, JoinTeamRequest, JoinTeamResponse, LeaveTeamRequest,
    LeaveTeamResponse, Role, RolePermission, TeamResponse, UserInfo,
};
use autometrics::autometrics;
use derive_new::new;
use std::str::FromStr;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use tracing::info;
use uuid::Uuid;

#[derive(new)]
pub struct TeamServiceImpl {
    pub(self) team_repository: Arc<dyn TeamRepository>,
    pub(self) role_repository: Arc<dyn RoleRepository>,
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

        let team_id = match Uuid::from_str(get_request.team_id.as_str()) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid team id"));
            }
        };

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
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn get_user_teams(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetAllTeamsResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let metadata = request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(Status::unauthenticated("Unauthenticated".to_string()));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();

        let user_id = match Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

        match team_repository.get_user_teams(&user_id) {
            Ok(teams) => {
                let teams: Vec<TeamResponse> = teams
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
                    .collect();

                Ok(Response::new(GetAllTeamsResponse { teams }))
            }
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn get_all_can_join(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetAllTeamsResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let metadata = request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(Status::unauthenticated("Unauthenticated".to_string()));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();

        let user_id = match Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

        match team_repository.get_all_can_join(&user_id) {
            Ok(teams) => {
                let teams: Vec<TeamResponse> = teams
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
                    .collect();

                Ok(Response::new(GetAllTeamsResponse { teams }))
            }
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn create(
        &self,
        request: Request<CreateTeamRequest>,
    ) -> Result<Response<CreateTeamResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let metadata = request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(Status::unauthenticated("Unauthenticated".to_string()));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();

        let sign_up_request = request.into_inner();

        let new_team = &mut TeamInformation::from(sign_up_request);

        let user_id = match Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

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

        match team_repository.create(new_team) {
            Ok(created_team) => {
                match role_repository.create(&admin_role) {
                    Ok(value) => info!("Role: {} for team: {}", created_team.id, value.name),
                    Err(e) => {
                        return Err(Status::internal(format!(
                            "Internal Server Error: {}",
                            e.message
                        )));
                    }
                }

                let creator = &TeamMember {
                    id: Uuid::now_v7(),
                    user_id,
                    team_id: created_team.id,
                    role_id: admin_role.id,
                };

                match team_repository.join(creator) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(Status::internal(format!(
                            "Internal Server Error: {}",
                            e.message
                        )));
                    }
                }

                match role_repository.create(&manager_role) {
                    Ok(value) => info!("Role: {} for team: {}", created_team.id, value.name),
                    Err(e) => {
                        return Err(Status::internal(format!(
                            "Internal Server Error: {}",
                            e.message
                        )));
                    }
                }

                match role_repository.create(&member_role) {
                    Ok(value) => info!("Role: {} for team: {}", created_team.id, value.name),
                    Err(e) => {
                        return Err(Status::internal(format!(
                            "Internal Server Error: {}",
                            e.message
                        )));
                    }
                }

                Ok(Response::new(CreateTeamResponse {
                    message: String::from("Team successfully created"),
                    team_id: created_team.id.to_string(),
                }))
            }
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn join(
        &self,
        request: Request<JoinTeamRequest>,
    ) -> Result<Response<JoinTeamResponse>, Status> {
        let team_repository = self.team_repository.clone();
        let role_repository = self.role_repository.clone();

        let metadata = request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(Status::unauthenticated("Unauthenticated".to_string()));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();
        let join_request = request.into_inner();

        let user_id = match Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

        let team_id = match Uuid::from_str(join_request.team_id.as_str()) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid team id"));
            }
        };

        let role_id = match role_repository.get_lowest_priority(&team_id) {
            Ok(value) => value.id,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Internal Server Error: {}",
                    e.message
                )));
            }
        };

        let new_member = &TeamMember {
            id: Uuid::now_v7(),
            user_id,
            team_id,
            role_id,
        };

        match team_repository.join(new_member) {
            Ok(_) => Ok(Response::new(JoinTeamResponse {
                message: String::from("Successfully joined to team"),
            })),
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn get_roles(
        &self,
        request: Request<GetTeamRolesRequest>,
    ) -> Result<Response<GetTeamRolesResponse>, Status> {
        let role_request = request.into_inner();
        let role_repository = self.role_repository.clone();

        let team_id = match Uuid::from_str(role_request.team_id.as_str()) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid team id"));
            }
        };

        match role_repository.get_all_for_team(&team_id) {
            Ok(value) => Ok(Response::new(GetTeamRolesResponse {
                roles: value
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
            })),
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn change_role(
        &self,
        request: Request<ChangeTeamRole>,
    ) -> Result<Response<Role>, Status> {
        let role_repository = self.role_repository.clone();

        let metadata = request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(Status::unauthenticated("Unauthenticated".to_string()));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();

        let called_user_id = match Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

        let change_role_request = request.into_inner();

        let user_id = match Uuid::from_str(change_role_request.user_id.as_str()) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

        let role_id = match Uuid::from_str(change_role_request.role_id.as_str()) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid team id"));
            }
        };

        match role_repository.update(&role_id, &user_id) {
            Ok(value) => {
                let called_role = match role_repository
                    .get_by_team_and_user_id(&value.team_id, &called_user_id)
                {
                    Ok(value) => value,
                    Err(e) => {
                        return Err(Status::internal(format!(
                            "Internal Server Error: {}",
                            e.message
                        )));
                    }
                };

                if !called_role.can_create_roles {
                    return Err(Status::permission_denied(
                        "You don' have a permission to do that".to_string(),
                    ));
                }

                Ok(Response::new(Role {
                    role_name: value.name,
                    priority: value.priority,
                    permission: Some(RolePermission {
                        can_add_task: value.can_add_task,
                        can_assign_task: value.can_assign_task,
                        can_approve_task: value.can_approve_task,
                        can_invite_in_team: value.can_invite_in_team,
                        can_create_roles: value.can_create_roles,
                    }),
                }))
            }
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
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
