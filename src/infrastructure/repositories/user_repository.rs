use std::sync::Arc;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use derive_new::new;
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use tracing::error;

use crate::domain::error::Error;
use crate::domain::error::Result;
use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;
use crate::domain::repositories::user::UserRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::user_information::UserInformationDiesel;
use crate::infrastructure::repositories::get_pool::GetPool;

#[derive(Clone, new)]
pub struct UserRepositoryImpl<'a> {
    pool: Arc<DBConn>,
    argon2: Arc<Argon2<'a>>,
}

impl GetPool for UserRepositoryImpl<'_> {}

impl UserRepository for UserRepositoryImpl<'_> {
    fn create(&self, new_user_information: &UserInformation) -> Result<UserInformation> {
        use crate::infrastructure::schema::user_information::dsl::user_information;

        let mut conn = Self::get_pool(&self.pool).unwrap();
        let mut hashed_user_info = new_user_information.clone();

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = match self
            .argon2
            .hash_password(hashed_user_info.password.as_bytes(), &salt)
        {
            Ok(pass) => pass.to_string(),
            Err(e) => {
                error!("{:?}", e);
                return Err(Error::RepositoryError);
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
                Err(Error::RepositoryError)
            }
        }
    }

    fn login(&self, login_information: &LoginInformation) -> Result<UserInformation> {
        use crate::infrastructure::schema::user_information::dsl::*;

        let mut conn = Self::get_pool(&self.pool).unwrap();

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
                        return Err(Error::RepositoryError);
                    }
                };

                match self
                    .argon2
                    .verify_password(login_information.password.as_bytes(), &parsed_hash)
                {
                    Ok(_) => Ok(UserInformation::from(user_inf)),
                    Err(_) => Err(Error::RepositoryError),
                }
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Error::RepositoryError)
            }
        }
    }
}
