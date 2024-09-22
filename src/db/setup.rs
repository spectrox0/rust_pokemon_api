use dotenvy::dotenv; 
use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Arc;

pub fn setup_db() -> Arc<r2d2::Pool<ConnectionManager<PgConnection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");
    Arc::new(pool)
}