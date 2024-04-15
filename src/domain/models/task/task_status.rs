#[derive(Clone, PartialEq, Eq)]
pub enum TaskStatus {
    InProgress,
    Paused,
    Finished,
    Canceled,
}

impl From<TaskStatus> for String {
    fn from(value: TaskStatus) -> String {
        match value {
            TaskStatus::InProgress => String::from("in progress"),
            TaskStatus::Paused => String::from("paused"),
            TaskStatus::Finished => String::from("finished"),
            TaskStatus::Canceled => String::from("canceled")
        }
    }
}
