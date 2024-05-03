use uuid::Uuid;

use crate::domain::error::Result;
use crate::domain::models::team::team_information::TeamInformation;
use crate::domain::models::team::team_leave::TeamLeave;
use crate::domain::models::team::team_member::TeamMember;

pub trait TeamRepository: Send + Sync {
    fn get(&self, team_id: &Uuid) -> Result<TeamInformation>;
    fn get_all_can_join(&self, user_id: &Uuid) -> Result<Vec<TeamInformation>>;
    fn get_user_teams(&self, user_id: &Uuid) -> Result<Vec<TeamInformation>>;
    fn create(&self, new_team_information: &TeamInformation) -> Result<TeamInformation>;
    fn join(&self, new_team_member: &TeamMember) -> Result<TeamMember>;
    fn leave(&self, team_leave: &TeamLeave) -> Result<()>;
}
