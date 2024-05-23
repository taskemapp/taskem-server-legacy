use std::sync::Arc;

use autometrics::autometrics;
use derive_new::new;
use tonic::{Request, Response, Status};
use tracing::log::error;

use crate::auth::auth_server::Auth;
use crate::auth::{
    LoginRequest, LoginResponse, LogoutRequest, LogoutResponse, SignUpResponse, SignupRequest,
};
use crate::core::regex::CachedRegexValidator;
use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;
use crate::domain::repositories::session::RedisSessionRepository;
use crate::domain::repositories::user::UserRepository;

#[derive(new)]
pub struct AuthServiceImpl {
    pub(self) regex_cache: Arc<CachedRegexValidator>,
    pub(self) user_repository: Arc<dyn UserRepository>,
    pub(self) redis_repository: Arc<dyn RedisSessionRepository>,
}

#[autometrics]
#[tonic::async_trait]
impl Auth for AuthServiceImpl {
    async fn sign_up(
        &self,
        request: Request<SignupRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        let sign_up_request = request.into_inner();

        if self
            .regex_cache
            .check_email(&sign_up_request.email)
            .is_err()
        {
            return Err(Status::invalid_argument("Invalid email"));
        }

        let user_repository = self.user_repository.clone();

        user_repository
            .create(&UserInformation::from(sign_up_request.clone()))
            .map_err(|e| {
                error!("{:?}", e);
                Status::internal("User creation failed")
            })?;

        Ok(Response::new(SignUpResponse {
            message: String::from("User successfully created"),
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let login_request = request.into_inner();

        if self.regex_cache.check_email(&login_request.email).is_err() {
            return Err(Status::invalid_argument("Invalid email"));
        }

        let user_repository = self.user_repository.clone();

        match user_repository.login(&LoginInformation::from(login_request)) {
            Ok(value) => {
                let session = self.redis_repository.create(&value.id).unwrap();
                return Ok(Response::new(LoginResponse {
                    user_name: value.user_name,
                    message: "User successfully logged".to_string(),
                    session_id: session,
                }));
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Status::not_found("User not found"))
            }
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
            Err(e) => Err(Status::internal(format!("Internal Server Error: {}", e))),
        }
    }
}
