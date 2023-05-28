// api/src/bin/main.rs

use axum::{
    routing::{get, get_service},
    Router, Server,
};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let _db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    /*
       let conn = Database::connect(db_url)
           .await
           .expect("Database connection failed");
       Migrator::up(&conn, None).await.unwrap();
    */

    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    let state = AppState { templates }; //, conn };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            ))), /*
                 .handle_error(|error: std::io::Error| async move {
                     (
                         StatusCode::INTERNAL_SERVER_ERROR,
                         format!("Unhandled internal error: {error}"),
                     )
                 }),
                  */
        )
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    templates: Tera,
    //conn: DatabaseConnection,
}
