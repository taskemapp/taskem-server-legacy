-- Your SQL goes here
create table team_information
(
    id          uuid primary key                      not null,
    name        varchar(25) unique                    not null check ( length(name) > 3 ),
    description text                                  not null,
    creator     uuid references user_information (id) not null
)
