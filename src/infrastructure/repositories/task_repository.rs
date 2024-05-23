use std::sync::Arc;

use derive_new::new;
use diesel::{
    insert_into, update, BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper,
};
use tracing::error;
use uuid::Uuid;

use crate::domain::error::Error;
use crate::domain::error::Result;
use crate::domain::models::task::task_assign::TaskAssign;
use crate::domain::models::task::task_information::TaskInformation;
use crate::domain::models::task::task_status::TaskStatus;
use crate::domain::models::user::user_information::UserInformation;
use crate::domain::repositories::task::TaskRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::task_assign::TaskAssignDiesel;
use crate::infrastructure::models::task_information::TaskInformationDiesel;
use crate::infrastructure::models::task_status::TaskStatusDiesel;
use crate::infrastructure::models::user_information::UserInformationDiesel;
use crate::infrastructure::repositories::get_pool::GetPool;
use crate::infrastructure::repositories::map_from::MapFrom;
use crate::infrastructure::schema::user_information;

#[derive(Clone, new)]
pub struct TaskRepositoryImpl {
    pool: Arc<DBConn>,
}

impl MapFrom for TaskRepositoryImpl {}

impl GetPool for TaskRepositoryImpl {}

impl TaskRepository for TaskRepositoryImpl {
    fn get(&self, task_id: &Uuid) -> Result<TaskInformation> {
        use crate::infrastructure::schema::task_information::dsl::task_information;
        use crate::infrastructure::schema::task_information::id;

        let mut conn = Self::get_pool(&self.pool).unwrap();

        let result = task_information
            .select(TaskInformationDiesel::as_select())
            .filter(id.eq(task_id))
            .first(&mut conn);

        match result {
            Ok(value) => {
                let query = TaskAssignDiesel::belonging_to(&value)
                    .inner_join(user_information::table)
                    .select(UserInformationDiesel::as_select())
                    .load(&mut conn);

                let users: Vec<UserInformation> = self.map_from(query).unwrap();

                let mut task_info = TaskInformation::from(value);
                task_info.assigned_users = users;

                Ok(task_info)
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn get_all(&self) -> Result<Vec<TaskInformation>> {
        use crate::infrastructure::schema::task_information::dsl::name;
        use crate::infrastructure::schema::task_information::dsl::task_information;

        let mut conn = Self::get_pool(&self.pool).unwrap();

        let result = task_information
            .select(TaskInformationDiesel::as_select())
            .limit(1000)
            .order_by(name)
            .load(&mut conn);

        match result {
            Ok(value) => {
                let tasks: Vec<TaskInformation> = value
                    .iter()
                    .map(|task_info| {
                        let mut task = TaskInformation::from(task_info);

                        let query = TaskAssignDiesel::belonging_to(&task_info)
                            .inner_join(user_information::table)
                            .select(UserInformationDiesel::as_select())
                            .load(&mut conn);

                        let users: Vec<UserInformation> = self.map_from(query).unwrap();

                        task.assigned_users = users;
                        task
                    })
                    .collect();

                Ok(tasks)
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn get_all_for_team(&self, id_team: &Uuid) -> Result<Vec<TaskInformation>> {
        use crate::infrastructure::schema::task_information::dsl::task_information;
        use crate::infrastructure::schema::task_information::dsl::team_id;
        use crate::infrastructure::schema::task_information::end_timestamp;
        let mut conn = Self::get_pool(&self.pool).unwrap();

        let result = task_information
            .select(TaskInformationDiesel::as_select())
            .limit(1000)
            .filter(team_id.eq(id_team))
            .order_by(end_timestamp)
            .load(&mut conn);

        match result {
            Ok(value) => {
                let tasks: Vec<TaskInformation> = value
                    .iter()
                    .map(|task_info| {
                        let mut task = TaskInformation::from(task_info);

                        let query = TaskAssignDiesel::belonging_to(&task_info)
                            .inner_join(user_information::table)
                            .select(UserInformationDiesel::as_select())
                            .load(&mut conn);

                        let users: Vec<UserInformation> = self.map_from(query).unwrap();

                        task.assigned_users = users;
                        task
                    })
                    .collect();

                Ok(tasks)
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn get_all_for_user(&self, id_user: &Uuid) -> Result<Vec<TaskInformation>> {
        use crate::infrastructure::schema::task_assign::dsl::task_assign;
        use crate::infrastructure::schema::task_assign::dsl::user_id;
        use crate::infrastructure::schema::task_information::dsl::id;
        use crate::infrastructure::schema::task_information::dsl::task_information;
        use crate::infrastructure::schema::task_information::end_timestamp;
        let mut conn = Self::get_pool(&self.pool).unwrap();

        let assigned_result = task_assign
            .select(TaskAssignDiesel::as_select())
            .filter(user_id.eq(id_user))
            .load(&mut conn);

        let task_assigned: Vec<Uuid> = match assigned_result {
            Ok(value) => value.iter().map(|e| e.task_id).collect(),
            Err(e) => {
                error!("{:?}", e);
                return Err(Error::RepositoryError);
            }
        };

        let result = task_information
            .select(TaskInformationDiesel::as_select())
            .limit(1000)
            .filter(id.eq_any(task_assigned))
            .order_by(end_timestamp)
            .load(&mut conn);

        match result {
            Ok(value) => {
                let tasks: Vec<TaskInformation> = value
                    .iter()
                    .map(|task_info| {
                        let mut task = TaskInformation::from(task_info);
                        let query = TaskAssignDiesel::belonging_to(&task_info)
                            .inner_join(user_information::table)
                            .select(UserInformationDiesel::as_select())
                            .load(&mut conn);

                        let users: Vec<UserInformation> = self.map_from(query).unwrap();

                        task.assigned_users = users;
                        task
                    })
                    .collect();

                Ok(tasks)
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn create(&self, new_task_information: &TaskInformation) -> Result<TaskInformation> {
        use crate::infrastructure::schema::task_information::dsl::task_information;

        let new_task = TaskInformationDiesel::from(new_task_information.clone());
        let mut conn = Self::get_pool(&self.pool).unwrap();

        let team_result = insert_into(task_information)
            .values(new_task)
            .get_result::<TaskInformationDiesel>(&mut conn);

        match team_result {
            Ok(created_task) => Ok(TaskInformation::from(created_task)),
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn assign(&self, new_task_assign: &TaskAssign) -> Result<TaskAssign> {
        use crate::infrastructure::schema::task_assign::dsl::task_assign;
        use crate::infrastructure::schema::task_information::dsl::id;
        use crate::infrastructure::schema::task_information::dsl::status;
        use crate::infrastructure::schema::task_information::dsl::task_information;

        let assign_task = TaskAssignDiesel::from(new_task_assign.clone());
        let mut conn = Self::get_pool(&self.pool).unwrap();

        let task_assign_result = insert_into(task_assign)
            .values(assign_task)
            .get_result::<TaskAssignDiesel>(&mut conn);

        match task_assign_result {
            Ok(assigned) => {
                let update_status_result = update(task_information)
                    .filter(id.eq(assigned.task_id))
                    .set(status.eq(TaskStatusDiesel::from(TaskStatus::InProgress)))
                    .execute(&mut conn);

                match update_status_result {
                    Ok(_) => Ok(TaskAssign::from(assigned)),
                    Err(e) => {
                        error!("{:?}", e);
                        Err(Error::RepositoryError)
                    }
                }
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }

    fn complete(&self, task_id: &Uuid) -> Result<TaskInformation> {
        use crate::infrastructure::schema::task_information::dsl::id;
        use crate::infrastructure::schema::task_information::dsl::status;
        use crate::infrastructure::schema::task_information::dsl::task_information;

        let mut conn = Self::get_pool(&self.pool).unwrap();

        let task = update(task_information)
            .filter(id.eq(task_id))
            .set(status.eq(TaskStatusDiesel::from(TaskStatus::Finished)))
            .get_result::<TaskInformationDiesel>(&mut conn)
            .map_err(|e| {
                error!("{:?}", e);
                Error::RepositoryError
            })?;

        Ok(TaskInformation::from(task))
    }
}
