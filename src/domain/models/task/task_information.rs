use crate::domain::models::task::task_status::TaskStatus;
use crate::domain::models::user::user_information::UserInformation;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct TaskInformation {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) created_timestamp: i64,
    pub(crate) end_timestamp: i64,
    pub(crate) team_id: Uuid,
    pub(crate) status: TaskStatus,
    pub(crate) creator: Uuid,
    pub(crate) assigned_users: Vec<UserInformation>,
}
