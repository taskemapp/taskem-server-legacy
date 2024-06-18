mod api;
pub mod app;
mod core;

mod common;
mod container;
mod domain;
mod infrastructure;

mod auth {
    tonic::include_proto!("auth");
}

mod team {
    tonic::include_proto!("team");
}

mod task {
    tonic::include_proto!("task");
}

mod profile {
    tonic::include_proto!("profile");
}
