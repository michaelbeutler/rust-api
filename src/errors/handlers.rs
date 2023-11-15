use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use uuid::Uuid;

use rocket::Request;

use crate::helpers::response::{ErrorBuilder, GenericResponse};

#[catch(default)]
pub fn default_catcher(
    status: Status,
    _req: &Request<'_>,
) -> status::Custom<Json<GenericResponse>> {
    status::Custom(
        status,
        GenericResponse::new(status)
            .add_error(
                ErrorBuilder::default()
                    .trace_id(Uuid::new_v4())
                    .code(format!("HTTP-{}", status.code).to_string())
                    .message(status.reason().unwrap_or_default().to_string())
                    .details(None)
                    .help(Some(
                        format!(
                            "https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/{}",
                            status.code
                        )
                        .to_string(),
                    ))
                    .build()
                    .unwrap(),
            )
            .to_json(),
    )
}
