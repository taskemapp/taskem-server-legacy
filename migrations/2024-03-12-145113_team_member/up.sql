-- Your SQL goes here
create table team_member
(
    id      uuid primary key                      not null,
    user_id uuid references user_information (id) not null,
    team_id uuid references team_information (id) not null,
    role_id uuid references team_role (id)        not null,
    constraint unique_user_team_combination unique (user_id, team_id)
)