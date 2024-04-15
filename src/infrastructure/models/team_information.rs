use crate::domain::models::team::team_information::TeamInformation;
use crate::infrastructure::models::user_information::UserInformationDiesel;
use crate::infrastructure::schema::team_information;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Associations, Selectable, PartialEq, Eq)]
#[diesel(table_name = team_information)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(UserInformationDiesel, foreign_key = creator))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TeamInformationDiesel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub creator: Uuid,
}

impl From<TeamInformationDiesel> for TeamInformation {
    fn from(value: TeamInformationDiesel) -> TeamInformation {
        TeamInformation {
            id: value.id,
            name: value.name,
            description: value.description,
            creator: value.creator,
            members: Vec::new(),
        }
    }
}

impl From<&TeamInformationDiesel> for TeamInformation {
    fn from(value: &TeamInformationDiesel) -> TeamInformation {
        TeamInformation {
            id: value.id,
            name: value.name.clone(),
            description: value.description.clone(),
            creator: value.creator,
            members: Vec::new(),
        }
    }
}

impl From<TeamInformation> for TeamInformationDiesel {
    fn from(value: TeamInformation) -> Self {
        TeamInformationDiesel {
            id: value.id,
            name: value.name,
            description: value.description,
            creator: value.creator,
        }
    }
}
