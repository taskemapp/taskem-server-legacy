use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;
use crate::domain::repositories::error::RepositoryError;
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::repositories::user::UserRepository;
use crate::infrastructure::models::user_information::UserInformationDiesel;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use db::get_pool::GetPool;
use db::pool::postgresql::PostgresDBConn;
use derive_new::new;
use diesel::ExpressionMethods;
use diesel::{insert_into, QueryDsl, RunQueryDsl, SelectableHelper};
use std::sync::Arc;
use tracing::error;

#[derive(Clone, new)]
pub struct UserRepositoryImpl<'a> {
    pool: Arc<PostgresDBConn>,
    argon2: Arc<Argon2<'a>>,
}

impl GetPool for UserRepositoryImpl<'_> {}

impl UserRepository for UserRepositoryImpl<'_> {
    fn create(&self, new_user_information: &UserInformation) -> RepositoryResult<UserInformation> {
        use db::schema::user_information::dsl::user_information;

        let mut conn = match Self::get_pool(&self.pool) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError { message: e.message });
            }
        };
        let mut hashed_user_info = new_user_information.clone();

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = match self
            .argon2
            .hash_password(hashed_user_info.password.as_bytes(), &salt)
        {
            Ok(pass) => pass.to_string(),
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError {
                    message: "Failed password hashing".to_string(),
                });
            }
        };

        hashed_user_info.password = password_hash;

        let new_user_information_diesel = UserInformationDiesel::from(hashed_user_info);
        let user_result = insert_into(user_information)
            .values(new_user_information_diesel)
            .get_result::<UserInformationDiesel>(&mut conn);

        match user_result {
            Ok(created_user) => Ok(UserInformation::from(created_user)),
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: e.to_string(),
                })
            }
        }
    }

    fn login(&self, login_information: &LoginInformation) -> RepositoryResult<UserInformation> {
        use db::schema::user_information::dsl::email;
        use db::schema::user_information::dsl::user_information;

        let mut conn = match Self::get_pool(&self.pool) {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}", e);
                return Err(RepositoryError { message: e.message });
            }
        };

        let user = user_information
            .select(UserInformationDiesel::as_select())
            .filter(email.eq(&login_information.email))
            .first(&mut conn);

        match user {
            Ok(user_inf) => {
                let parsed_hash = match PasswordHash::new(&user_inf.password) {
                    Ok(pass) => pass,
                    Err(e) => {
                        error!("{:?}", e);
                        return Err(RepositoryError {
                            message: "Incorrect password".to_string(),
                        });
                    }
                };

                match self
                    .argon2
                    .verify_password(login_information.password.as_bytes(), &parsed_hash)
                {
                    Ok(_) => Ok(UserInformation::from(user_inf)),
                    Err(_) => Err(RepositoryError {
                        message: "Incorrect password".to_string(),
                    }),
                }
            }
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: "User doesn't exist".to_string(),
                })
            }
        }
    }
}
