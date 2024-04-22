-- Your SQL goes here
create table task_assign
(
    id      uuid primary key                      not null,
    task_id uuid references task_information (id) not null,
    user_id uuid references user_information (id) not null,
    constraint unique_user_task_combination unique (task_id, user_id)
)