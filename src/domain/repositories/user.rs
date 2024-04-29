use crate::domain::error::Result;
use crate::domain::models::user::login_information::LoginInformation;
use crate::domain::models::user::user_information::UserInformation;

pub trait UserRepository: Send + Sync {
    fn create(&self, user_information: &UserInformation) -> Result<UserInformation>;
    fn login(&self, login_information: &LoginInformation) -> Result<UserInformation>;
}
