use crate::auth::{LoginRequest, SignupRequest};
use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;
use uuid::Uuid;

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
