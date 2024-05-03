use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInformation {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) user_name: String,
    pub(crate) profile_image: Option<String>,
    pub(crate) password: String,
}
