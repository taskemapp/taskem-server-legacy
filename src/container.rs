use crate::api::middlewares::auth::AuthMiddlewareLayer;
use crate::api::services::auth::AuthServiceImpl;
use crate::api::services::file::FileServiceData;
use crate::api::services::profile::ProfileServiceImpl;
use crate::api::services::task::TaskServiceImpl;
use crate::api::services::team::TeamServiceImpl;
use crate::auth::auth_server::AuthServer;
use crate::core::regex::CachedRegexValidator;
use crate::domain::constants::POSTGRESQL_DB_URI;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::databases::redis::redis_pool;
use crate::infrastructure::repositories::file_repository::FileRepositoryImpl;
use crate::infrastructure::repositories::role_repository::RoleRepositoryImpl;
use crate::infrastructure::repositories::session_repository::RedisSessionRepositoryImpl;
use crate::infrastructure::repositories::task_repository::TaskRepositoryImpl;
use crate::infrastructure::repositories::team_repository::TeamRepositoryImpl;
use crate::infrastructure::repositories::user_repository::UserRepositoryImpl;
use crate::profile::profile_server::ProfileServer;
use crate::task::task_server::TaskServer;
use crate::team::team_server::TeamServer;
use argon2::Argon2;
use autometrics::autometrics;
use aws_sdk_s3::Client;
use diesel_migrations::{FileBasedMigrations, MigrationHarness};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tower::layer::util::{Identity, Stack};
use tower::timeout::TimeoutLayer;

#[derive(Clone)]
pub struct Container {
    pub auth_server: AuthServer<AuthServiceImpl>,
    pub team_server: TeamServer<TeamServiceImpl>,
    pub task_server: TaskServer<TaskServiceImpl>,
    pub profile_server: ProfileServer<ProfileServiceImpl>,
    pub file_service_data: FileServiceData,
    pub auth_layer: AuthMiddlewareLayer,
    pub layer: Stack<AuthMiddlewareLayer, Stack<TimeoutLayer, Identity>>,
}

#[autometrics]
impl Container {
    pub async fn new() -> Self {
        let regex_cache = Arc::new({
            let mut to_compile = CachedRegexValidator::default();
            to_compile.compile_all();
            to_compile
        });

        let database_url = env::var(POSTGRESQL_DB_URI)
            .unwrap_or_else(|_| panic!("{value} must be set", value = POSTGRESQL_DB_URI));

        let pool = Arc::new(db_pool());

        let migrations_dir = "migrations";
        let migrations =
            FileBasedMigrations::from_path(migrations_dir).expect("Can't get migrations");

        db_pool()
            .get()
            .unwrap()
            .run_pending_migrations(migrations)
            .unwrap();

        let redis_pool = Arc::new(redis_pool().unwrap());
        let argon2 = Arc::new(Argon2::default());

        let aws_config = aws_config::load_from_env().await;
        let s3_client = Arc::new(Client::new(&aws_config));

        let buckets = s3_client.list_buckets().send().await.unwrap();

        if buckets.buckets().is_empty() {
            s3_client
                .create_bucket()
                .bucket("users")
                .send()
                .await
                .unwrap();

            s3_client
                .create_bucket()
                .bucket("teams")
                .send()
                .await
                .unwrap();
        }

        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone(), argon2.clone()));
        let team_repository = Arc::new(TeamRepositoryImpl::new(pool.clone()));
        let task_repository = Arc::new(TaskRepositoryImpl::new(pool.clone()));
        let role_repository = Arc::new(RoleRepositoryImpl::new(pool));
        let file_repository = Arc::new(FileRepositoryImpl::new(s3_client));
        let redis_session_repository = Arc::new(RedisSessionRepositoryImpl::new(redis_pool));

        let auth_layer = AuthMiddlewareLayer::new(redis_session_repository.clone());
        let layer = tower::ServiceBuilder::new()
            .timeout(Duration::from_secs(30))
            .layer(auth_layer.clone())
            .into_inner();

        let auth_service = AuthServiceImpl::new(
            regex_cache,
            user_repository.clone(),
            redis_session_repository,
        );
        let team_service = TeamServiceImpl::new(team_repository, role_repository.clone());
        let task_service = TaskServiceImpl::new(task_repository, role_repository);
        let profile_service = ProfileServiceImpl::new(
            file_repository.clone(),
            user_repository.clone(),
            String::from("localhost/file"),
        );

        let auth_server = AuthServer::new(auth_service);
        let team_server = TeamServer::new(team_service);
        let task_server = TaskServer::new(task_service);
        let profile_server = ProfileServer::new(profile_service);

        let file_service_data = FileServiceData::new(file_repository, user_repository);

        Container {
            auth_server,
            team_server,
            task_server,
            profile_server,
            auth_layer,
            file_service_data,
            layer,
        }
    }
}
