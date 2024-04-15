use uuid::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct TeamMember {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) team_id: Uuid,
    pub(crate) role_id: Uuid,
}
