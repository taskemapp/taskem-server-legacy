use crate::domain::models::team::team_role::TeamRole;
use crate::infrastructure::models::team_information::TeamInformationDiesel;
use crate::infrastructure::schema::team_role;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Insertable, Queryable, Identifiable, Associations, Selectable, Debug, PartialEq, Eq)]
#[diesel(table_name = team_role)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(TeamInformationDiesel, foreign_key = team_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TeamRoleDiesel {
    pub id: Uuid,
    pub team_id: Uuid,
    pub name: String,
    pub priority: i32,
    pub can_add_task: bool,
    pub can_assign_task: bool,
    pub can_approve_task: bool,
    pub can_invite_in_team: bool,
    pub can_create_roles: bool,
}

impl From<TeamRoleDiesel> for TeamRole {
    fn from(value: TeamRoleDiesel) -> TeamRole {
        TeamRole {
            id: value.id,
            name: value.name,
            team_id: value.team_id,
            priority: value.priority,
            can_add_task: value.can_add_task,
            can_assign_task: value.can_assign_task,
            can_approve_task: value.can_approve_task,
            can_invite_in_team: value.can_invite_in_team,
            can_create_roles: value.can_create_roles,
        }
    }
}

impl From<&TeamRoleDiesel> for TeamRole {
    fn from(value: &TeamRoleDiesel) -> TeamRole {
        TeamRole {
            id: value.id,
            name: value.name.clone(),
            team_id: value.team_id,
            priority: value.priority,
            can_add_task: value.can_add_task,
            can_assign_task: value.can_assign_task,
            can_approve_task: value.can_approve_task,
            can_invite_in_team: value.can_invite_in_team,
            can_create_roles: value.can_create_roles,
        }
    }
}

impl From<TeamRole> for TeamRoleDiesel {
    fn from(value: TeamRole) -> Self {
        TeamRoleDiesel {
            id: value.id,
            name: value.name,
            team_id: value.team_id,
            priority: value.priority,
            can_add_task: value.can_add_task,
            can_assign_task: value.can_assign_task,
            can_approve_task: value.can_approve_task,
            can_invite_in_team: value.can_invite_in_team,
            can_create_roles: value.can_create_roles,
        }
    }
}
