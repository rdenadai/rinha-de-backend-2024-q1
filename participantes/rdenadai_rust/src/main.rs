mod routes;
mod schemas;
use std::str::FromStr;

use actix_web::middleware::Logger;
use sqlx::postgres::PgPoolOptions;
use actix_web::{middleware::Compress, web::Data, App, HttpServer};

use dotenvy::dotenv;
use routes::register::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url: String = String::from_str("postgres://admin:123@localhost:5432/rinha").unwrap();

    match PgPoolOptions::new().max_connections(20).connect(&database_url).await {
        Ok(pool) => {
            // Database migrations
            println!("Database connected");

            // Run http server
            println!("Starting server");
            HttpServer::new(move || {
                App::new()
                    .wrap(Compress::default())
                    .wrap(Logger::default())
                    .app_data(Data::new(pool.clone()))
                    .configure(configure)
            })
            .bind(("0.0.0.0", 8080))?
            .run()
            .await
        }
        Err(e) => panic!("Error connecting to database: {:?}", e),
    }
}
