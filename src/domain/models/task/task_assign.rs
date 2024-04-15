use uuid::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct TaskAssign {
    pub(crate) id: Uuid,
    pub(crate) task_id: Uuid,
    pub(crate) user_id: Uuid,
}
