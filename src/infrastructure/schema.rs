// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "task_status"))]
    pub struct TaskStatus;
}

diesel::table! {
    task_assign (id) {
        id -> Uuid,
        task_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TaskStatus;

    task_information (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        description -> Text,
        created_timestamp -> Int8,
        end_timestamp -> Int8,
        status -> TaskStatus,
        team_id -> Uuid,
        creator -> Uuid,
    }
}

diesel::table! {
    team_information (id) {
        id -> Uuid,
        #[max_length = 25]
        name -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        #[max_length = 255]
        header_image -> Nullable<Varchar>,
        description -> Text,
        creator -> Uuid,
    }
}

diesel::table! {
    team_member (id) {
        id -> Uuid,
        user_id -> Uuid,
        team_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    team_role (id) {
        id -> Uuid,
        team_id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        priority -> Int4,
        can_add_task -> Bool,
        can_assign_task -> Bool,
        can_approve_task -> Bool,
        can_invite_in_team -> Bool,
        can_create_roles -> Bool,
    }
}

diesel::table! {
    user_information (id) {
        id -> Uuid,
        email -> Varchar,
        #[max_length = 255]
        profile_image -> Nullable<Varchar>,
        #[max_length = 25]
        user_name -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(task_assign -> task_information (task_id));
diesel::joinable!(task_assign -> user_information (user_id));
diesel::joinable!(task_information -> team_information (team_id));
diesel::joinable!(task_information -> user_information (creator));
diesel::joinable!(team_information -> user_information (creator));
diesel::joinable!(team_member -> team_information (team_id));
diesel::joinable!(team_member -> team_role (role_id));
diesel::joinable!(team_member -> user_information (user_id));
diesel::joinable!(team_role -> team_information (team_id));

diesel::allow_tables_to_appear_in_same_query!(
    task_assign,
    task_information,
    team_information,
    team_member,
    team_role,
    user_information,
);
