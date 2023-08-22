#[macro_use]
extern crate diesel;

mod config;
mod models;
mod modules;
// mod modules;
// mod jwt_auth;
mod middlewares;
mod routes;
mod schema;

// use actix_cors::Cors;
// use actix_web::http::header;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use env_logger::Env;
use log::info;
// use lumau_backend::config::config::read_config;
use config::config::Config;
use config::types::{DatabaseConnection, Pool};
use modules::{admin_users, auth, version};

pub struct AppState {
    // db: Pool<ConnectionManager<SqliteConnection>>,
    db: Pool,
    env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Config
    // let config = read_config();

    let config = Config::init();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting Server on {}:{}", config.host, config.port);
    info!(
        "See version on http://{}:{}/api/version",
        config.host, config.port
    );
    // let address = format!("127.0.0.1:{}", port);
    // info!("Starting Our Server at {}", address);

    // Database
    let database_pool = Pool::builder()
        .build(ConnectionManager::<DatabaseConnection>::new(
            &config.database,
        ))
        .unwrap();

    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:3000")
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![
        //         header::CONTENT_TYPE,
        //         header::AUTHORIZATION,
        //         header::ACCEPT,
        //     ])
        //     .supports_credentials();
        App::new()
            // .app_data(web::Data::new(database_pool.clone()))
            .app_data(web::Data::new(AppState {
                db: database_pool.clone(),
                env: config.clone(),
            }))
            .route("/", web::get().to(routes::home))
            .route("/addlink", web::post().to(routes::add_link))
            .route("/getlinks", web::get().to(routes::get_links))
            .service(
                web::scope("/api")
                    .configure(version::routes::config)
                    .configure(admin_users::routes::config)
                    .configure(auth::routes::config),
            )
        //.wrap(cors)
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}
