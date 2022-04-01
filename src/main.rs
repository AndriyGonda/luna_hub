use std::collections::HashMap;
use env_logger::Env;
use log::info;
use actix_web::{web, App, HttpResponse, HttpServer};
use config::{Config, Value};
use luna_hub::routeconfig;


fn load_application_config() -> HashMap<String, Value> {
    const CONFIG_FILE_NAME: &str = "Config.toml";
    const APPLICATION_TABLE: &str = "application";

    let settings = Config::builder()
        .add_source(config::File::with_name(CONFIG_FILE_NAME))
        .build()
        .unwrap();
    let application_config = settings.get_table(APPLICATION_TABLE).unwrap();
    application_config.clone()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = load_application_config();
    let bind_host = config
        .get("host")
        .expect("Unable to get bind host")
        .to_string();

    let bind_port = config
        .get("port")
        .expect("Unable to get bind port")
        .clone()
        .into_int()
        .expect("Unable to parse bind port") as u16;
    info!("Starting http server on host {:?} and port {:?}", bind_host, bind_port);
    HttpServer::new(|| {
        App::new()
            .configure(routeconfig::configure)
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind((bind_host, bind_port))?
    .run()
    .await
}
