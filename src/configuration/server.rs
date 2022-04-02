use config::Value;
use std::collections::HashMap;

const BIND_HOST: &'static str = "host";
const FAILED_GET_HOST: &'static str = "Unable to get bind host";
const BIND_PORT: &'static str = "port";
const FAILED_GET_PORT: &'static str = "Unable to get bind port";
const FAILED_PARSE_PORT: &'static str = "Unable to parse bind port";

pub fn load_server_properties(config: &HashMap<String, Value>) -> (String, u16) {
    let bind_host = config.get(BIND_HOST).expect(FAILED_GET_HOST).to_string();

    let bind_port = config
        .get(BIND_PORT)
        .expect(FAILED_GET_PORT)
        .clone()
        .into_int()
        .expect(FAILED_PARSE_PORT) as u16;
    (bind_host, bind_port)
}
