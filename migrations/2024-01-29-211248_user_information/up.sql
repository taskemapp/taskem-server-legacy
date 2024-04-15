-- Your SQL goes here
create extension if not exists "uuid-ossp";

create table
    user_information
(
    id        uuid primary key   not null,
    email     varchar unique     not null,
    user_name varchar(25) unique not null check ( length(user_name) > 3 ),
    password  varchar            not null
)