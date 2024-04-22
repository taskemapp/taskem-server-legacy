use crate::pool::postgresql::PostgresDBConn;
use diesel_migrations::{FileBasedMigrations, MigrationHarness};

pub fn run(pool: &PostgresDBConn) {
    let migrations_dir = "migrations";
    let migrations = FileBasedMigrations::from_path(migrations_dir).expect("Can't get migrations");
    pool.get()
        .expect("Can't get a connection from pool")
        .run_pending_migrations(migrations)
        .unwrap();
}
