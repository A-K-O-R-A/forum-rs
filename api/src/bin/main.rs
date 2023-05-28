// api/src/bin/main.rs
use dotenvy::dotenv;
use std::env;

fn main() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
}
