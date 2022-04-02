use diesel::{pg::PgConnection, r2d2, r2d2::ConnectionManager};
pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

const FAILED_CREATE_POOL: &'static str = "Failed to create database pool";

pub fn configure_database(url: &String) -> Pool {
    let manager = ConnectionManager::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect(FAILED_CREATE_POOL);
    pool
}
