use crate::domain::error::RepositoryError;
use crate::domain::models::team::team_information::TeamInformation;
use crate::domain::models::team::team_leave::TeamLeave;
use crate::domain::models::team::team_member::TeamMember;
use crate::domain::models::user::user_information::UserInformation;
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::repositories::team::TeamRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::team_information::TeamInformationDiesel;
use crate::infrastructure::models::team_members::TeamMemberDiesel;
use crate::infrastructure::models::user_information::UserInformationDiesel;
use crate::infrastructure::repositories::get_pool::GetPool;
use crate::infrastructure::repositories::map_from::MapFrom;
use crate::infrastructure::schema::user_information;
use derive_new::new;
use diesel::{
    delete, insert_into, BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Clone, new)]
pub struct TeamRepositoryImpl {
    pool: Arc<DBConn>,
}

impl MapFrom for TeamRepositoryImpl {}

impl GetPool for TeamRepositoryImpl {}

impl TeamRepository for TeamRepositoryImpl {
    fn get(&self, id_team: &Uuid) -> RepositoryResult<TeamInformation> {
        use crate::infrastructure::schema::team_information::dsl::team_information;
        use crate::infrastructure::schema::team_information::id;

        let mut conn = match Self::get_pool(&self.pool) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError { message: e.message });
            }
        };

        let result = team_information
            .select(TeamInformationDiesel::as_select())
            .filter(id.eq(id_team))
            .first(&mut conn);

        match result {
            Ok(value) => {
                let query = TeamMemberDiesel::belonging_to(&value)
                    .inner_join(user_information::table)
                    .select(UserInformationDiesel::as_select())
                    .load(&mut conn);

                let users: Vec<UserInformation> = match query {
                    Ok(value) => value.iter().map(UserInformation::from).collect(),
                    Err(e) => {
                        error!("{:?}", e);
                        return Err(RepositoryError {
                            message: e.to_string(),
                        });
                    }
                };

                let mut team_info = TeamInformation::from(value);
                team_info.members = users;

                Ok(team_info)
            }
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: e.to_string(),
                })
            }
        }
    }

    fn get_all_can_join(&self, id_user: &Uuid) -> RepositoryResult<Vec<TeamInformation>> {
        use crate::infrastructure::schema::team_information::dsl::creator;
        use crate::infrastructure::schema::team_information::dsl::id;
        use crate::infrastructure::schema::team_information::dsl::team_information;
        use crate::infrastructure::schema::team_member::dsl::team_member;
        use crate::infrastructure::schema::team_member::dsl::user_id;

        let mut conn = match Self::get_pool(&self.pool) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError { message: e.message });
            }
        };

        let teams_can_join_query = team_member
            .select(TeamMemberDiesel::as_select())
            .filter(user_id.ne(id_user))
            .load(&mut conn);

        let teams_can_join: Vec<uuid::Uuid> = match teams_can_join_query {
            Ok(value) => value.iter().map(|team| team.team_id).collect(),
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError {
                    message: e.to_string(),
                });
            }
        };

        let result = team_information
            .filter(id.eq_any(&teams_can_join))
            .filter(creator.ne(id_user))
            .select(TeamInformationDiesel::as_select())
            .limit(1000)
            .load(&mut conn);

        match result {
            Ok(value) => {
                let teams: Vec<TeamInformation> = value
                    .iter()
                    .map(|team_info| {
                        let query = TeamMemberDiesel::belonging_to(&team_info)
                            .inner_join(user_information::table)
                            .select(UserInformationDiesel::as_select())
                            .load(&mut conn);

                        let users: Vec<UserInformation> = query
                            .unwrap_or_else(|_| {
                                panic!("Failed to get users for a team: {}", &team_info.id)
                            })
                            .iter()
                            .map(UserInformation::from)
                            .collect();

                        let mut team = TeamInformation::from(team_info);

                        team.members = users;
                        team
                    })
                    .collect();
                Ok(teams)
            }
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: e.to_string(),
                })
            }
        }
    }

    fn get_user_teams(&self, id_user: &Uuid) -> RepositoryResult<Vec<TeamInformation>> {
        use crate::infrastructure::schema::team_information::dsl::id;
        use crate::infrastructure::schema::team_information::dsl::team_information;
        use crate::infrastructure::schema::team_member::dsl::team_member;
        use crate::infrastructure::schema::team_member::dsl::user_id;

        let mut conn = match Self::get_pool(&self.pool) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError { message: e.message });
            }
        };

        let teams_membership_query = team_member
            .select(TeamMemberDiesel::as_select())
            .filter(user_id.eq(id_user))
            .load(&mut conn);

        let teams_membership: Vec<uuid::Uuid> = match teams_membership_query {
            Ok(value) => value.iter().map(|team| team.team_id).collect(),
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError {
                    message: e.to_string(),
                });
            }
        };

        let result = team_information
            .filter(id.eq_any(teams_membership))
            .select(TeamInformationDiesel::as_select())
            .limit(1000)
            .load(&mut conn);

        match result {
            Ok(value) => {
                let teams: Vec<TeamInformation> = value
                    .iter()
                    .map(|team_info| {
                        let query = TeamMemberDiesel::belonging_to(&team_info)
                            .inner_join(user_information::table)
                            .select(UserInformationDiesel::as_select())
                            .load(&mut conn);

                        let users: Vec<UserInformation> = query
                            .unwrap_or_else(|_| {
                                panic!("Failed to get users for a team: {}", &team_info.id)
                            })
                            .iter()
                            .map(UserInformation::from)
                            .collect();

                        let mut team = TeamInformation::from(team_info);

                        team.members = users;
                        team
                    })
                    .collect();
                Ok(teams)
            }
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: e.to_string(),
                })
            }
        }
    }

    fn create(&self, new_team_information: &TeamInformation) -> RepositoryResult<TeamInformation> {
        use crate::infrastructure::schema::team_information::dsl::team_information;

        let new_team = TeamInformationDiesel::from(new_team_information.clone());
        let mut conn = match Self::get_pool(&self.pool) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError { message: e.message });
            }
        };

        let team_result = insert_into(team_information)
            .values(new_team)
            .get_result::<TeamInformationDiesel>(&mut conn);

        match team_result {
            Ok(created_team) => Ok(TeamInformation::from(created_team)),
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: e.to_string(),
                })
            }
        }
    }

    fn join(&self, new_team_member: &TeamMember) -> RepositoryResult<TeamMember> {
        use crate::infrastructure::schema::team_member::dsl::team_member;

        let new_member = TeamMemberDiesel::from(new_team_member.clone());
        let mut conn = match Self::get_pool(&self.pool) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError { message: e.message });
            }
        };

        let team_member_result = insert_into(team_member)
            .values(new_member)
            .get_result::<TeamMemberDiesel>(&mut conn);

        match team_member_result {
            Ok(joined_member) => Ok(TeamMember::from(joined_member)),
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: e.to_string(),
                })
            }
        }
    }

    fn leave(&self, leave_information: &TeamLeave) -> RepositoryResult<()> {
        use crate::infrastructure::schema::team_member::dsl::*;

        let mut conn = match Self::get_pool(&self.pool) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError { message: e.message });
            }
        };

        let leave_result = delete(team_member)
            .filter(team_id.eq(leave_information.team_id))
            .filter(user_id.eq(leave_information.user_id))
            .execute(&mut conn);

        match leave_result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: e.to_string(),
                })
            }
        }
    }
}
