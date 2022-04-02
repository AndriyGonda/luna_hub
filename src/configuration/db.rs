embed_migrations!();

use diesel::{pg::PgConnection, r2d2, r2d2::ConnectionManager};

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn configure_database(url: &String) -> Pool {
    const FAILED_CREATE_POOL: &'static str = "Failed to create database pool";
    const MIGRATION_FAILED: &'static str = "Failed to migrate";

    let manager = ConnectionManager::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect(FAILED_CREATE_POOL);

    embedded_migrations::run(&pool.get().expect(MIGRATION_FAILED)).ok();

    pool
}
