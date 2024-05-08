use uuid::Uuid;

use crate::domain::error::Result;
use crate::domain::models::task::task_assign::TaskAssign;
use crate::domain::models::task::task_information::TaskInformation;

pub trait TaskRepository: Send + Sync {
    fn get(&self, task_id: &Uuid) -> Result<TaskInformation>;
    fn get_all(&self) -> Result<Vec<TaskInformation>>;
    fn get_all_for_team(&self, team_id: &Uuid) -> Result<Vec<TaskInformation>>;
    fn get_all_for_user(&self, user_id: &Uuid) -> Result<Vec<TaskInformation>>;
    fn create(&self, new_task_information: &TaskInformation) -> Result<TaskInformation>;
    fn assign(&self, new_task_assign: &TaskAssign) -> Result<TaskAssign>;
    fn complete(&self, task_id: &Uuid) -> Result<TaskInformation>;
}
