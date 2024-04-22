use crate::auth::auth_server::AuthServer;
use argon2::Argon2;
use std::sync::Arc;
use std::time::Duration;

use db::pool::postgresql::postgres_pool;
use db::pool::redis::redis_pool;
use tower::layer::util::{Identity, Stack};
use tower::timeout::TimeoutLayer;

use crate::api::middlewares::auth::AuthMiddlewareLayer;
use crate::api::services::auth::AuthServiceImpl;
use crate::core::regex::CachedRegexValidator;
use crate::infrastructure::repositories::session_repository::RedisSessionRepositoryImpl;
use crate::infrastructure::repositories::user_repository::UserRepositoryImpl;

pub struct Container {
    pub auth_server: AuthServer<AuthServiceImpl>,
    pub layer: Stack<AuthMiddlewareLayer, Stack<TimeoutLayer, Identity>>,
}

impl Container {
    pub fn new() -> Self {
        let regex_cache = Arc::new({
            let mut to_compile = CachedRegexValidator::default();
            to_compile.compile_all();
            to_compile
        });

        let pool = Arc::new(postgres_pool());
        let redis_pool = Arc::new(redis_pool());
        let argon2 = Arc::new(Argon2::default());

        db::migration::run(&pool);

        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone(), argon2.clone()));
        let redis_session_repository = Arc::new(RedisSessionRepositoryImpl::new(redis_pool));

        let layer = tower::ServiceBuilder::new()
            .timeout(Duration::from_secs(30))
            .layer(AuthMiddlewareLayer::new(redis_session_repository.clone()))
            .into_inner();

        let auth_service =
            AuthServiceImpl::new(regex_cache, user_repository, redis_session_repository);

        let auth_server = AuthServer::new(auth_service);

        Container { auth_server, layer }
    }
}

impl Default for Container {
    fn default() -> Self {
        Container::new()
    }
}
