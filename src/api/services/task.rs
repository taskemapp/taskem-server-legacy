use crate::domain::constants::MIDDLEWARE_AUTH_SESSION_KEY;
use crate::domain::models::task::task_assign::TaskAssign;
use crate::domain::models::task::task_information::TaskInformation;
use crate::domain::repositories::task::TaskRepository;
use crate::task::{AssignTaskRequest, CreateTaskRequest, GetAllResponse, GetTaskRequest, GetTeamTasksRequest, TaskResponse};
use derive_new::new;
use std::str::FromStr;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use tracing::{error};
use uuid::{Uuid};
use crate::domain::repositories::role::RoleRepository;
use crate::task::task_server::Task;

#[derive(new)]
pub struct TaskServiceImpl {
    pub(self) task_repository: Arc<dyn TaskRepository>,
    pub(self) role_repository: Arc<dyn RoleRepository>,
}

#[async_trait]
impl Task for TaskServiceImpl {
    async fn create(&self, request: Request<CreateTaskRequest>) -> Result<Response<()>, Status> {
        let task_repository = self.task_repository.clone();
        let role_repository = self.role_repository.clone();

        let metadata = request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(Status::unauthenticated("Unauthenticated".to_string()));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();

        let user_id = match Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

        let create_request = request.into_inner();

        let team_id = match Uuid::from_str(&create_request.team_id) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid task id"));
            }
        };

        match role_repository.get_by_team_and_user_id(&team_id, &user_id) {
            Ok(value) => {
                if !value.can_create_roles {
                    return Err(
                        Status::permission_denied("Can't create task"),
                    );
                }
            }
            Err(e) => {
                return Err(Status::internal(format!(
                    "Internal Server Error: {}",
                    e.message
                )));
            }
        }

        let mut task_information = TaskInformation::from(create_request);
        task_information.creator = user_id;

        match task_repository.create(&task_information) {
            Ok(_) => Ok(Response::new(())),
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn get_all(&self, _: Request<()>) -> Result<Response<GetAllResponse>, Status> {
        let task_repository = self.task_repository.clone();

        match task_repository.get_all() {
            Ok(value) => Ok(Response::new(GetAllResponse {
                tasks: value.iter().map(TaskResponse::from).collect(),
            })),
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn get_all_for_team(&self, request: Request<GetTeamTasksRequest>) -> Result<Response<GetAllResponse>, Status> {
        let get_team_tasks_request = request.into_inner();
        let task_repository = self.task_repository.clone();

        let team_id = match Uuid::from_str(&get_team_tasks_request.team_id) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid task id"));
            }
        };

        match task_repository.get_all_for_team(&team_id) {
            Ok(value) => Ok(Response::new(GetAllResponse {
                tasks: value.iter().map(TaskResponse::from).collect(),
            })),
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn get_all_for_user(&self, request: Request<()>) -> Result<Response<GetAllResponse>, Status> {
        let task_repository = self.task_repository.clone();

        let metadata = request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(Status::unauthenticated("Unauthenticated".to_string()));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();

        let user_id = match Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

        match task_repository.get_all_for_user(&user_id) {
            Ok(value) => Ok(Response::new(GetAllResponse {
                tasks: value.iter().map(TaskResponse::from).collect(),
            })),
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn get(
        &self,
        request: Request<GetTaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let get_request = request.into_inner();
        let task_repository = self.task_repository.clone();

        let task_id = match Uuid::from_str(&get_request.id) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid task id"));
            }
        };

        match task_repository.get(&task_id) {
            Ok(value) => {
                let response = TaskResponse::from(value);

                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }

    async fn assign(
        &self,
        request: Request<AssignTaskRequest>,
    ) -> Result<Response<TaskResponse>, Status> {
        let task_repository = self.task_repository.clone();
        let role_repository = self.role_repository.clone();

        let metadata = request.metadata().clone();

        if !metadata.contains_key(MIDDLEWARE_AUTH_SESSION_KEY) {
            return Err(Status::unauthenticated("Unauthenticated".to_string()));
        }

        let metadata_value = metadata.get(MIDDLEWARE_AUTH_SESSION_KEY).unwrap();
        let assign_request = request.into_inner();

        let creator_user_id = match Uuid::from_str(
            metadata_value
                .to_str()
                .expect("Failed to convert metadata value to str"),
        ) {
            Ok(value) => value,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid user id"));
            }
        };

        let user_id = match Uuid::from_str(&assign_request.user_id) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(Status::invalid_argument("Invalid task id"));
            }
        };

        let task_id = match Uuid::from_str(&assign_request.task_id) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(Status::invalid_argument("Invalid task id"));
            }
        };

        let team_id = match task_repository.get(&task_id) {
            Ok(value) => value.team_id,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Internal Server Error: {}",
                    e.message
                )));
            }
        };

        let creator_priority = match role_repository.get_by_team_and_user_id(&team_id, &creator_user_id) {
            Ok(value) => {
                if !value.can_assign_task {
                    return Err(
                        Status::permission_denied("Can't assign task"),
                    );
                }
                value.priority
            }
            Err(e) => {
                return Err(Status::internal(format!(
                    "Internal Server Error: {}",
                    e.message
                )));
            }
        };

        let assigned_priority = match role_repository.get_by_team_and_user_id(&team_id, &user_id) {
            Ok(value) => value.priority,
            Err(e) => {
                return Err(Status::internal(format!(
                    "Internal Server Error: {}",
                    e.message
                )));
            }
        };
        
        if assigned_priority < creator_priority {
            return Err(
                Status::permission_denied("Can't assign task"),
            );
        }
        
        let task_assign = TaskAssign {
            id: Uuid::now_v7(),
            task_id,
            user_id,
        };

        match task_repository.assign(&task_assign) {
            Ok(value) => {
                let assigned_task = match task_repository.get(&value.task_id) {
                    Ok(value) => value,
                    Err(e) => {
                        return Err(Status::internal(format!(
                            "Internal Server Error: {}",
                            e.message
                        )));
                    }
                };

                let response = TaskResponse::from(assigned_task);

                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }
}
