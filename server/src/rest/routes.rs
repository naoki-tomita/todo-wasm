use rocket::config::{Config, Environment};
use super::{systems, todos};

pub fn start_server() {
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(8001)
        .finalize().unwrap();

    rocket::custom(config)
        .mount(
            "/v1",
            routes![
                systems::ping,
                systems::register_data,
                systems::clear,
                todos::get,
                todos::post,
                todos::put
            ],
        )
        .launch();
}
