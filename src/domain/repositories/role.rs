


use crate::domain::models::team::team_role::TeamRole;
use crate::domain::repositories::repository::RepositoryResult;
use uuid::Uuid;

pub trait RoleRepository: Send + Sync {
    fn get(&self, role_id: &Uuid) -> RepositoryResult<TeamRole>;
    fn get_lowest_priority(&self, team_id: &Uuid) -> RepositoryResult<TeamRole>;
    fn get_by_team_and_user_id(&self, team_id: &Uuid, user_id: &Uuid)
        -> RepositoryResult<TeamRole>;
    fn get_all_for_team(&self, team_id: &Uuid) -> RepositoryResult<Vec<TeamRole>>;
    fn create(&self, new_team_role: &TeamRole) -> RepositoryResult<TeamRole>;
    fn update(&self, role_id: &Uuid, user_id: &Uuid) -> RepositoryResult<TeamRole>;
}
