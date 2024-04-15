use crate::domain::models::user::user_information::UserInformation;
use crate::infrastructure::schema::user_information;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Selectable, Debug, PartialEq, Eq)]
#[diesel(table_name = user_information)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInformationDiesel {
    pub id: Uuid,
    pub password: String,
    pub user_name: String,
    pub email: String,
}

impl From<UserInformationDiesel> for UserInformation {
    fn from(value: UserInformationDiesel) -> UserInformation {
        UserInformation {
            id: value.id,
            password: value.password,
            user_name: value.user_name,
            email: value.email,
        }
    }
}

impl From<&UserInformationDiesel> for UserInformation {
    fn from(value: &UserInformationDiesel) -> UserInformation {
        UserInformation {
            id: value.id,
            password: value.password.clone(),
            user_name: value.user_name.clone(),
            email: value.email.clone(),
        }
    }
}

impl From<UserInformation> for UserInformationDiesel {
    fn from(value: UserInformation) -> Self {
        UserInformationDiesel {
            id: value.id,
            password: value.password,
            user_name: value.user_name,
            email: value.email,
        }
    }
}
