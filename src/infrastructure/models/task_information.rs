use crate::domain::models::task::task_information::TaskInformation;
use crate::infrastructure::models::team_information::TeamInformationDiesel;
use crate::infrastructure::schema::task_information;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;
use crate::domain::models::task::task_status::TaskStatus;
use crate::infrastructure::models::task_status::TaskStatusDiesel;
use crate::infrastructure::models::user_information::UserInformationDiesel;

#[derive(Insertable, Queryable, Identifiable, Associations, Selectable, PartialEq, Eq)]
#[diesel(table_name = task_information)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(TeamInformationDiesel, foreign_key = team_id))]
#[diesel(belongs_to(UserInformationDiesel, foreign_key = creator))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TaskInformationDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_timestamp: i64,
    pub end_timestamp: i64,
    pub status: TaskStatusDiesel,
    pub team_id: Uuid,
    pub creator: Uuid,
}

impl From<TaskInformationDiesel> for TaskInformation {
    fn from(value: TaskInformationDiesel) -> TaskInformation {
        TaskInformation {
            id: value.id,
            name: value.name,
            description: value.description,
            created_timestamp: value.created_timestamp,
            end_timestamp: value.end_timestamp,
            team_id: value.team_id,
            status: TaskStatus::from(value.status),
            creator: value.creator,
            assigned_users: vec![],
        }
    }
}

impl From<&TaskInformationDiesel> for TaskInformation {
    fn from(value: &TaskInformationDiesel) -> TaskInformation {
        TaskInformation {
            id: value.id,
            name: value.name.clone(),
            description: value.description.clone(),
            created_timestamp: value.created_timestamp,
            end_timestamp: value.end_timestamp,
            team_id: value.team_id,
            status: TaskStatus::from(value.status.clone()),
            creator: value.creator,
            assigned_users: vec![],
        }
    }
}

impl From<TaskInformation> for TaskInformationDiesel {
    fn from(value: TaskInformation) -> Self {
        TaskInformationDiesel {
            id: value.id,
            name: value.name,
            description: value.description,
            created_timestamp: value.created_timestamp,
            end_timestamp: value.end_timestamp,
            status: TaskStatusDiesel::from(value.status),
            team_id: value.team_id,
            creator: value.creator,
        }
    }
}
