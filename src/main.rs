use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::Env;
use log::{info};

use luna_hub::configuration::app;
use luna_hub::configuration::conf::Config;

const CONFIG_FILENAME: &'static str = "Config.toml";


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = Config::from_file(CONFIG_FILENAME)?;
    info!(
            "Starting http server on host {:?} and port {:?}",
            config.application.host, config.application.port
        );
    let server = HttpServer::new(|| {
        App::new()
            .wrap(app::cors_configuration())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(app::configure)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind((config.application.host, config.application.port))?
    .run();
    server.await
}
