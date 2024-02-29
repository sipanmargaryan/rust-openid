use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db_pool(url: &str) -> Pool {
    
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    
    pool
}