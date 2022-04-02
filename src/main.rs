use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger::Env;
use log::info;

use luna_hub::configuration::{app, server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = app::load_application_config();

    let (bind_host, bind_port) = server::load_server_properties(&config);
    info!(
        "Starting http server on host {:?} and port {:?}",
        bind_host, bind_port
    );

    let server = HttpServer::new(|| {
        App::new()
            .wrap(app::cors_configuration())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(app::configure)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind((bind_host, bind_port))?
    .run();
    server.await
}
