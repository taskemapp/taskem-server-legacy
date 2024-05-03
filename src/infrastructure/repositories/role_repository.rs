use std::sync::Arc;

use derive_new::new;
use diesel::{insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::error;
use uuid::Uuid;

use crate::domain::error::Error;
use crate::domain::error::Result;
use crate::domain::models::team::team_role::TeamRole;
use crate::domain::repositories::role::RoleRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::team_members::TeamMemberDiesel;
use crate::infrastructure::models::team_role::TeamRoleDiesel;
use crate::infrastructure::repositories::get_pool::GetPool;
use crate::infrastructure::repositories::map_from::MapFrom;

#[derive(Clone, new)]
pub struct RoleRepositoryImpl {
    pool: Arc<DBConn>,
}

impl MapFrom for RoleRepositoryImpl {}

impl GetPool for RoleRepositoryImpl {}

impl RoleRepository for RoleRepositoryImpl {
    fn get(&self, _: &Uuid) -> Result<TeamRole> {
        todo!()
    }

    fn get_lowest_priority(&self, id_team: &Uuid) -> Result<TeamRole> {
        use crate::infrastructure::schema::team_role::dsl::team_id;
        use crate::infrastructure::schema::team_role::dsl::team_role;

        let mut conn = Self::get_pool(&self.pool).unwrap();

        let result = team_role
            .select(TeamRoleDiesel::as_select())
            .filter(team_id.eq(id_team))
            .load(&mut conn);

        match result {
            Ok(mut roles) => {
                roles.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());

                let lowest = match roles.first() {
                    Some(value) => value,
                    None => {
                        return Err(Error::RepositoryError);
                    }
                };

                Ok(TeamRole::from(lowest))
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn get_by_team_and_user_id(&self, id_team: &Uuid, id_user: &Uuid) -> Result<TeamRole> {
        use crate::infrastructure::schema::team_member::dsl::team_id;
        use crate::infrastructure::schema::team_member::dsl::team_member;
        use crate::infrastructure::schema::team_member::dsl::user_id;
        use crate::infrastructure::schema::team_role::dsl::id;
        use crate::infrastructure::schema::team_role::dsl::team_role;

        let mut conn = Self::get_pool(&self.pool).unwrap();

        let result = team_member
            .select(TeamMemberDiesel::as_select())
            .filter(team_id.eq(id_team))
            .filter(user_id.eq(id_user))
            .first(&mut conn);

        match result {
            Ok(member) => {
                let query = team_role
                    .select(TeamRoleDiesel::as_select())
                    .filter(id.eq(member.role_id))
                    .first(&mut conn);

                let role = match query {
                    Ok(value) => value,
                    Err(e) => {
                        error!("{:?}", e);
                        return Err(Error::RepositoryError);
                    }
                };

                Ok(TeamRole::from(role))
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn get_all_for_team(&self, id_team: &Uuid) -> Result<Vec<TeamRole>> {
        use crate::infrastructure::schema::team_role::dsl::team_id;
        use crate::infrastructure::schema::team_role::dsl::team_role;

        let mut conn = Self::get_pool(&self.pool).unwrap();

        let result = team_role
            .select(TeamRoleDiesel::as_select())
            .filter(team_id.eq(id_team))
            .load(&mut conn);

        match result {
            Ok(value) => Ok(value.iter().map(TeamRole::from).collect()),
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn create(&self, new_team_role: &TeamRole) -> Result<TeamRole> {
        use crate::infrastructure::schema::team_role::dsl::team_role;

        let mut conn = Self::get_pool(&self.pool).unwrap();

        let role = TeamRoleDiesel::from(new_team_role.clone());

        let result = insert_into(team_role)
            .values(role)
            .get_result::<TeamRoleDiesel>(&mut conn);

        match result {
            Ok(created_role) => Ok(TeamRole::from(created_role)),
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn update(&self, role_id: &Uuid, id_user: &Uuid) -> Result<TeamRole> {
        use crate::infrastructure::schema::team_member::dsl::team_id as member_team_id;
        use crate::infrastructure::schema::team_member::dsl::team_member;
        use crate::infrastructure::schema::team_member::dsl::user_id;
        use crate::infrastructure::schema::team_role::dsl::id;
        use crate::infrastructure::schema::team_role::dsl::team_id;
        use crate::infrastructure::schema::team_role::dsl::team_role;

        let mut conn = Self::get_pool(&self.pool).unwrap();

        let role_query = team_role
            .select(TeamRoleDiesel::as_select())
            .filter(id.eq(role_id))
            .first(&mut conn);

        let role = match role_query {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(Error::RepositoryError);
            }
        };

        let member_query = team_member
            .select(TeamMemberDiesel::as_select())
            .filter(user_id.eq(id_user))
            .filter(member_team_id.eq(role.team_id))
            .first(&mut conn);

        let mut update_member = match member_query {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(Error::RepositoryError);
            }
        };

        if role.team_id != update_member.team_id {
            return Err(Error::RepositoryError);
        }

        update_member.role_id = role.id;

        let updated_member_query = update(
            team_member
                .filter(user_id.eq(id_user))
                .filter(member_team_id.eq(role.team_id)),
        )
        .set(update_member)
        .get_result::<TeamMemberDiesel>(&mut conn);

        let updated_member = match updated_member_query {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(Error::RepositoryError);
            }
        };

        let set_role_query = team_role
            .select(TeamRoleDiesel::as_select())
            .filter(id.eq(updated_member.role_id))
            .filter(team_id.eq(updated_member.team_id))
            .first(&mut conn);

        let set_role = match set_role_query {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(Error::RepositoryError);
            }
        };

        Ok(TeamRole::from(set_role))
    }
}
