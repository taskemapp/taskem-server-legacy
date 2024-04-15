use crate::auth::auth_server::Auth;
use crate::auth::{
    LoginRequest, LoginResponse, LogoutRequest, LogoutResponse, SignUpResponse, SignupRequest,
};
use crate::domain::constants::EMAIL;
use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;
use crate::domain::repositories::session::RedisSessionRepository;
use crate::domain::repositories::user::UserRepository;
use crate::core::regex::match_regex;
use derive_new::new;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::log::error;

#[derive(new)]
pub struct AuthServiceImpl {
    pub(self) user_repository: Arc<dyn UserRepository>,
    pub(self) redis_repository: Arc<dyn RedisSessionRepository>,
}

#[tonic::async_trait]
impl Auth for AuthServiceImpl {
    async fn sign_up(
        &self,
        request: Request<SignupRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        let sign_up_request = request.into_inner();
        if match_regex(EMAIL, &sign_up_request.email).is_err() {
            return Err(Status::invalid_argument("Invalid email"));
        }

        let user_repository = self.user_repository.clone();

        return match user_repository.create(&UserInformation::from(sign_up_request)) {
            Ok(_) => Ok(Response::new(SignUpResponse {
                message: String::from("User successfully created"),
            })),
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        };
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let login_request = request.into_inner();
        if match_regex(EMAIL, &login_request.email).is_err() {
            return Err(Status::invalid_argument("Invalid email"));
        }

        let user_repository = self.user_repository.clone();

        match user_repository.login(&LoginInformation::from(login_request)) {
            Ok(value) => {
                let session = self.redis_repository.create(&value.id).unwrap();
                return Ok(Response::new(LoginResponse {
                    success: true,
                    message: "User successfully logged".to_string(),
                    session_id: session,
                }));
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Status::not_found("User not found"))
            },
        }
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        let logout_request = request.into_inner();
        match self
            .redis_repository
            .remove_session(&logout_request.session_id)
        {
            Ok(_) => Ok(Response::new(LogoutResponse {
                message: "User session removed".to_string(),
            })),
            Err(e) => Err(Status::internal(format!(
                "Internal Server Error: {}",
                e.message
            ))),
        }
    }
}
