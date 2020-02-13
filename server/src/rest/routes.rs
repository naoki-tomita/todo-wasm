use super::{systems, todos};

pub fn start_server() {
    rocket::ignite()
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
