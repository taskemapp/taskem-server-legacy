use uuid::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct TeamLeave {
    pub(crate) user_id: Uuid,
    pub(crate) team_id: Uuid,
}
