use std::ops::Add;
use std::str::FromStr;

use chrono::{TimeDelta, Utc};
use prost_types::Timestamp;
use uuid::Uuid;

use crate::auth::{LoginRequest, SignupRequest};
use crate::domain::models::task::task_information::TaskInformation;
use crate::domain::models::task::task_status::TaskStatus;
use crate::domain::models::team::team_information::TeamInformation;
use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;
use crate::task::{CreateTaskRequest, TaskResponse};
use crate::team::{CreateTeamRequest, UserInfo};

impl From<SignupRequest> for UserInformation {
    fn from(value: SignupRequest) -> Self {
        UserInformation {
            id: Uuid::now_v7(),
            email: value.email,
            user_name: value.user_name,
            password: value.password,
        }
    }
}

impl From<LoginRequest> for LoginInformation {
    fn from(value: LoginRequest) -> Self {
        LoginInformation {
            email: value.email,
            password: value.password,
        }
    }
}

impl From<CreateTeamRequest> for TeamInformation {
    fn from(value: CreateTeamRequest) -> Self {
        TeamInformation {
            id: Uuid::now_v7(),
            name: value.name,
            description: value.description,
            creator: Uuid::default(),
            members: Vec::new(),
        }
    }
}

impl From<CreateTaskRequest> for TaskInformation {
    fn from(value: CreateTaskRequest) -> Self {
        TaskInformation {
            id: Uuid::now_v7(),
            name: value.name,
            description: value.description,
            created_timestamp: Utc::now().timestamp(),
            end_timestamp: match value.end_timestamp {
                None => Utc::now().add(TimeDelta::try_days(1).unwrap()).timestamp(),
                Some(value) => value.seconds,
            },
            team_id: Uuid::from_str(&value.team_id).expect("Failed to parse team id"),
            status: TaskStatus::Paused,
            creator: Uuid::default(),
            assigned_users: vec![],
        }
    }
}

impl From<TaskInformation> for TaskResponse {
    fn from(value: TaskInformation) -> Self {
        TaskResponse {
            id: value.id.to_string(),
            name: value.name,
            description: value.description,
            created_timestamp: Some(Timestamp {
                seconds: value.created_timestamp,
                nanos: 0,
            }),
            end_timestamp: Some(Timestamp {
                seconds: value.end_timestamp,
                nanos: 0,
            }),
            assigned_users: value
                .assigned_users
                .iter()
                .map(|assigned| UserInfo {
                    id: assigned.id.to_string(),
                    user_name: "".to_string(),
                    role: "".to_string(),
                })
                .collect(),
            status: value.status.into(),
        }
    }
}

impl From<&TaskInformation> for TaskResponse {
    fn from(value: &TaskInformation) -> Self {
        TaskResponse {
            id: value.id.to_string(),
            name: value.name.clone(),
            description: value.description.clone(),
            created_timestamp: Some(Timestamp {
                seconds: value.created_timestamp,
                nanos: 0,
            }),
            end_timestamp: Some(Timestamp {
                seconds: value.end_timestamp,
                nanos: 0,
            }),
            assigned_users: value
                .assigned_users
                .iter()
                .map(|assigned| UserInfo {
                    id: assigned.id.to_string(),
                    user_name: assigned.user_name.clone(),
                    role: "".to_string(),
                })
                .collect(),
            status: value.status.clone().into(),
        }
    }
}
