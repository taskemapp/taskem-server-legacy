use crate::domain::models::team::team_information::TeamInformation;
use crate::domain::models::team::team_leave::TeamLeave;
use crate::domain::models::team::team_member::TeamMember;
use crate::domain::repositories::repository::RepositoryResult;
use uuid::Uuid;

pub trait TeamRepository: Send + Sync {
    fn get(&self, team_id: &Uuid) -> RepositoryResult<TeamInformation>;
    fn get_all_can_join(&self, user_id: &Uuid) -> RepositoryResult<Vec<TeamInformation>>;
    fn get_user_teams(&self, user_id: &Uuid) -> RepositoryResult<Vec<TeamInformation>>;
    fn create(&self, new_team_information: &TeamInformation) -> RepositoryResult<TeamInformation>;
    fn join(&self, new_team_member: &TeamMember) -> RepositoryResult<TeamMember>;
    fn leave(&self, team_leave: &TeamLeave) -> RepositoryResult<()>;
}
