extern crate diesel;
mod json_serialization;
use actix_cors::Cors;
use actix_service::Service;
use actix_web::{web, App, HttpResponse, HttpServer};
use config::Config;
use diesel::{r2d2::ConnectionManager, PgConnection};
use futures::future::{ok, Either};
use routes::users_config;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod config;
mod jwt;
#[path = "./models/models.rs"]
pub mod models;
#[path = "./routes/routes.rs"]
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = "v1";

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    dotenv::dotenv().ok();

    let database_url = Config::new()
        .map
        .get("DB_URL")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

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
                let passed: bool;

                if *&req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = true;
                } else {
                    passed = false;
                    println!("{}", req.path());
                }

                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => {
                        let resp = HttpResponse::NotImplemented()
                            .body(format!("only {} API is supported", ALLOWED_VERSION));
                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };
                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .app_data(web::Data::new(pool.clone()))
            .configure(users_config)
            .wrap(cors)
        //.configure(patients_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
