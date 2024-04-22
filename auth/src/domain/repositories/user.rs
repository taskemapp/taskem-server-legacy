use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;
use crate::domain::repositories::repository::RepositoryResult;

pub trait UserRepository: Send + Sync {
    fn create(&self, user_information: &UserInformation) -> RepositoryResult<UserInformation>;
    fn login(&self, login_information: &LoginInformation) -> RepositoryResult<UserInformation>;
}
