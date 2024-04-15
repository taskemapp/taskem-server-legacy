use crate::domain::models::task::task_assign::TaskAssign;
use crate::infrastructure::models::task_information::TaskInformationDiesel;

use crate::infrastructure::models::user_information::UserInformationDiesel;
use crate::infrastructure::schema::task_assign;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Associations, Selectable, PartialEq, Eq)]
#[diesel(table_name = task_assign)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(TaskInformationDiesel, foreign_key = task_id))]
#[diesel(belongs_to(UserInformationDiesel, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TaskAssignDiesel {
    pub id: Uuid,
    pub task_id: Uuid,
    pub user_id: Uuid,
}

impl From<TaskAssignDiesel> for TaskAssign {
    fn from(value: TaskAssignDiesel) -> TaskAssign {
        TaskAssign {
            id: value.id,
            user_id: value.user_id,
            task_id: value.task_id,
        }
    }
}

impl From<TaskAssign> for TaskAssignDiesel {
    fn from(value: TaskAssign) -> Self {
        TaskAssignDiesel {
            id: value.id,
            user_id: value.user_id,
            task_id: value.task_id,
        }
    }
}
