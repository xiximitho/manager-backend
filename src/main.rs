extern crate diesel;
mod json_serialization;
use actix_cors::Cors;
use actix_service::Service;
use actix_web::{web, App, HttpServer};
use diesel::{r2d2::ConnectionManager, PgConnection};
use routes::{patients_config, users_config};
use std::env;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod jwt;
#[path = "./models/models.rs"]
pub mod models;
#[path = "./routes/routes.rs"]
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    //Inicia a pool de conexão
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .app_data(web::Data::new(pool.clone()))
            .configure(users_config)
            .wrap(cors)
            .configure(patients_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
