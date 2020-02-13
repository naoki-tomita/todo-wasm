use crate::drivers::todo_driver;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PongJson {
    pong: String,
}

#[get("/systems/ping")]
pub fn ping() -> Json<PongJson> {
    Json(PongJson {
        pong: "ok".to_string(),
    })
}

#[post("/test/register")]
pub fn register_data() -> Status {
    todo_driver::test_register_data();
    Status::Accepted
}

#[post("/test/clear")]
pub fn clear() -> Status {
    todo_driver::test_clear();
    Status::Accepted
}
