mod routes;
mod models;
use actix_cors::Cors;
use actix_web::{HttpServer, http::header, App, middleware::Logger};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use routes::{health_route::health_checker_handler, config::config};
struct AppState{
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("parse an url to db");
    let pool: Pool<Postgres> = match PgPoolOptions::new().max_connections(10).connect(&database_url).await
        {
            Ok(pool) => {
                println!("connection to db is sucsessful");
                pool
            }, 
            Err(err) => {
                println!("connection is denied {:?}", err);
                std::process::exit(1);
            },
        };
    println!("server started sucs!");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
            .supports_credentials();
        App::new()
            .app_data(actix_web::web::Data::new(AppState{db: pool.clone()}))
            .wrap(cors)
            .configure(config)
            .service(health_checker_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
