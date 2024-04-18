use crate::domain::models::task::task_assign::TaskAssign;
use crate::domain::models::task::task_information::TaskInformation;

use crate::domain::repositories::repository::RepositoryResult;
use uuid::Uuid;

pub trait TaskRepository: Send + Sync {
    fn get(&self, task_id: &Uuid) -> RepositoryResult<TaskInformation>;
    fn get_all(&self) -> RepositoryResult<Vec<TaskInformation>>;
    fn get_all_for_team(&self, team_id: &Uuid) -> RepositoryResult<Vec<TaskInformation>>;
    fn get_all_for_user(&self, user_id: &Uuid) -> RepositoryResult<Vec<TaskInformation>>;
    fn create(&self, new_task_information: &TaskInformation) -> RepositoryResult<TaskInformation>;
    fn assign(&self, new_task_assign: &TaskAssign) -> RepositoryResult<TaskAssign>;
    fn finish_task(&self, new_task_assign: &TaskAssign) -> RepositoryResult<TaskAssign>;
}
