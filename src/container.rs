use std::sync::Arc;
use std::time::Duration;
use argon2::Argon2;
use diesel_migrations::{FileBasedMigrations, MigrationHarness};

use tower::layer::util::{Identity, Stack};
use tower::timeout::TimeoutLayer;

use crate::api::middlewares::auth::AuthMiddlewareLayer;
use crate::api::services::auth::AuthServiceImpl;
use crate::api::services::task::TaskServiceImpl;
use crate::api::services::team::TeamServiceImpl;
use crate::auth::auth_server::AuthServer;
use crate::core::regex::CachedRegexValidator;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::databases::redis::redis_pool;
use crate::infrastructure::repositories::role_repository::RoleRepositoryImpl;
use crate::infrastructure::repositories::session_repository::RedisSessionRepositoryImpl;
use crate::infrastructure::repositories::task_repository::TaskRepositoryImpl;
use crate::infrastructure::repositories::team_repository::TeamRepositoryImpl;
use crate::infrastructure::repositories::user_repository::UserRepositoryImpl;
use crate::task::task_server::TaskServer;
use crate::team::team_server::TeamServer;

pub struct Container {
    pub auth_server: AuthServer<AuthServiceImpl>,
    // pub chat_server: ChatServiceServer<ChatServiceServerImpl>,
    pub team_server: TeamServer<TeamServiceImpl>,
    pub task_server: TaskServer<TaskServiceImpl>,
    pub layer: Stack<AuthMiddlewareLayer, Stack<TimeoutLayer, Identity>>,
}

impl Container {
    pub fn new() -> Self {
        let regex_cache = Arc::new({
            let mut to_compile = CachedRegexValidator::default();
            to_compile.compile_all();
            to_compile
        });
        
        let pool = Arc::new(db_pool());
        let redis_pool = Arc::new(redis_pool().unwrap());
        let argon2 = Arc::new(Argon2::default());

        let migrations_dir = "migrations";
        let migrations = FileBasedMigrations::from_path(migrations_dir).expect("Can't get migrations");

        db_pool().get().expect("Can't get a connection from pool").run_pending_migrations(migrations).unwrap();

        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone(), argon2.clone()));
        let team_repository = Arc::new(TeamRepositoryImpl::new(pool.clone()));
        let task_repository = Arc::new(TaskRepositoryImpl::new(pool.clone()));
        let role_repository = Arc::new(RoleRepositoryImpl::new(pool.clone()));
        let redis_session_repository = Arc::new(RedisSessionRepositoryImpl::new(redis_pool));
        
        let layer = tower::ServiceBuilder::new()
            .timeout(Duration::from_secs(30))
            .layer(AuthMiddlewareLayer::new(redis_session_repository.clone()))
            // .layer(GrpcWebLayer::new())
            // .layer(CorsLayer::permissive())
            .into_inner();

        // let chat_service = ChatServiceServerImpl::new(map);
        let auth_service = AuthServiceImpl::new(regex_cache, user_repository, redis_session_repository);
        let team_service = TeamServiceImpl::new(team_repository, role_repository.clone());
        let task_service = TaskServiceImpl::new(task_repository, role_repository);

        // let chat_server = ChatServiceServer::new(chat_service);
        let auth_server = AuthServer::new(auth_service);
        let team_server = TeamServer::new(team_service);
        let task_server = TaskServer::new(task_service);

        Container {
            auth_server,
            // chat_server,
            team_server,
            task_server,
            layer,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Container::new()
    }
}
