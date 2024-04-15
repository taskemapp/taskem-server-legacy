-- Your SQL goes here
create table team_role
(
    id                 uuid primary key                      not null,
    team_id            uuid references team_information (id) NOT NULL,
    name               varchar(50)                           NOT NULL,
    priority           int                                   NOT NULL,
    can_add_task       bool                                  not null,
    can_assign_task    boolean                               not null,
    can_approve_task   boolean                               not null,
    can_invite_in_team boolean                               not null,
    can_create_roles   boolean                               not null,
    constraint unique_team_id_name_combination unique (team_id, name)
);
