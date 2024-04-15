use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamRole {
    pub(crate) id: Uuid,
    pub(crate) team_id: Uuid,
    pub(crate) name: String,
    pub(crate) priority: i32,
    pub(crate) can_add_task: bool,
    pub(crate) can_assign_task: bool,
    pub(crate) can_approve_task: bool,
    pub(crate) can_invite_in_team: bool,
    pub(crate) can_create_roles: bool,
}
