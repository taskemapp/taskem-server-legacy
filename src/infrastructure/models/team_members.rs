use crate::domain::models::team::team_member::TeamMember;
use crate::infrastructure::models::team_information::TeamInformationDiesel;
use crate::infrastructure::models::team_role::TeamRoleDiesel;
use crate::infrastructure::models::user_information::UserInformationDiesel;
use crate::infrastructure::schema::team_member;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(
    Insertable, Queryable, AsChangeset, Selectable, Associations, Identifiable, PartialEq, Eq,
)]
#[diesel(table_name = team_member)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(UserInformationDiesel, foreign_key = user_id))]
#[diesel(belongs_to(TeamInformationDiesel, foreign_key = team_id))]
#[diesel(belongs_to(TeamRoleDiesel, foreign_key = role_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TeamMemberDiesel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub team_id: Uuid,
    pub role_id: Uuid,
}

impl From<TeamMemberDiesel> for TeamMember {
    fn from(value: TeamMemberDiesel) -> TeamMember {
        TeamMember {
            id: value.id,
            user_id: value.user_id,
            team_id: value.team_id,
            role_id: value.role_id,
        }
    }
}

impl From<TeamMember> for TeamMemberDiesel {
    fn from(value: TeamMember) -> Self {
        TeamMemberDiesel {
            id: value.id,
            user_id: value.user_id,
            team_id: value.team_id,
            role_id: value.role_id,
        }
    }
}
