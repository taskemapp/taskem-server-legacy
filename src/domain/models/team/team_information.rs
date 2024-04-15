use crate::domain::models::user::user_information::UserInformation;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamInformation {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) creator: Uuid,
    pub(crate) members: Vec<UserInformation>,
}
