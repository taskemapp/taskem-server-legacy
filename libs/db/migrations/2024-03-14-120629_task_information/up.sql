-- Your SQL goes here
CREATE TYPE task_status AS ENUM ('in progress', 'paused', 'finished', 'canceled');

create table task_information
(
    id                uuid primary key                      not null,
    name              varchar(50)                           not null check ( length(name) > 3 ),
    description       text                                  not null,
    created_timestamp bigint                                not null,
    end_timestamp     bigint                                not null check ( created_timestamp < end_timestamp ),
    status            task_status                           not null,
    team_id           uuid references team_information (id) not null,
    creator           uuid references user_information (id) not null
)