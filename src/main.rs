#[macro_use]
extern crate rocket;
extern crate derive_builder;

use authentication::jwt::Jwt;
use helpers::response::GenericResponse;
use rocket::{http::Status, serde::json::Json};

mod authentication;
mod errors;
mod helpers;
mod metrics;

#[get("/errors/<code>")]
fn forced_error(code: u16) -> Status {
    Status::new(code)
}

#[get("/")]
fn index(_jwt: Jwt) -> Json<GenericResponse<String>> {
    GenericResponse::ok("Hello World!".to_string()).to_json()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(metrics::PrometheusMetrics::default())
        .mount("/", routes![index, forced_error])
        .register("/", catchers![errors::handlers::default_catcher])
}
