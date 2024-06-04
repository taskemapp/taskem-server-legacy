use crate::domain::models::task::task_status::TaskStatus;
use crate::infrastructure::schema::sql_types::TaskStatus as TaskStatusScheme;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{AsExpression, FromSqlRow};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;

#[derive(Clone, Debug, FromSqlRow, AsExpression, PartialEq, Eq)]
#[diesel(sql_type = TaskStatusScheme)]
pub enum TaskStatusDiesel {
    InProgress,
    Paused,
    Finished,
    Canceled,
}

struct ParseEnumError {}

impl Debug for ParseEnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error while parsing enum TaskStatus")
    }
}

impl Display for ParseEnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error while parsing enum TaskStatus")
    }
}

impl Error for ParseEnumError {}

impl From<TaskStatusDiesel> for TaskStatus {
    fn from(value: TaskStatusDiesel) -> Self {
        match value {
            TaskStatusDiesel::InProgress => TaskStatus::InProgress,
            TaskStatusDiesel::Paused => TaskStatus::Paused,
            TaskStatusDiesel::Finished => TaskStatus::Finished,
            TaskStatusDiesel::Canceled => TaskStatus::Canceled,
        }
    }
}

impl From<TaskStatus> for TaskStatusDiesel {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::InProgress => TaskStatusDiesel::InProgress,
            TaskStatus::Paused => TaskStatusDiesel::Paused,
            TaskStatus::Finished => TaskStatusDiesel::Finished,
            TaskStatus::Canceled => TaskStatusDiesel::Canceled,
        }
    }
}

impl ToSql<TaskStatusScheme, Pg> for TaskStatusDiesel {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            TaskStatusDiesel::InProgress => out.write_all(b"in progress")?,
            TaskStatusDiesel::Paused => out.write_all(b"paused")?,
            TaskStatusDiesel::Finished => out.write_all(b"finished")?,
            TaskStatusDiesel::Canceled => out.write_all(b"canceled")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<TaskStatusScheme, Pg> for TaskStatusDiesel {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        let binding = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
        let status = binding.as_str();
        match status {
            "in progress" => Ok(TaskStatusDiesel::InProgress),
            "paused" => Ok(TaskStatusDiesel::Paused),
            "finished" => Ok(TaskStatusDiesel::Finished),
            "canceled" => Ok(TaskStatusDiesel::Canceled),
            _ => Err(Box::new(ParseEnumError {})),
        }
    }
}
