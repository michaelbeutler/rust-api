use derive_builder::Builder;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Default, Builder, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Metadata {
    pub page: u32,
    pub limit: u32,
    pub total: u32,
    pub pages: u32,
}

#[derive(Serialize, Default, Builder, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Error {
    pub trace_id: Uuid,
    pub code: String,
    pub message: String,
    pub details: Option<String>,
    pub help: Option<String>,
}

#[derive(Serialize, Builder)]
#[serde(crate = "rocket::serde")]
pub struct GenericResponse<T = ()> {
    pub status: u16,
    pub message: String,
    pub errors: Option<Vec<Error>>,
    pub data: Option<T>,
    pub timestamp: chrono::NaiveDateTime,
    pub metadata: Option<Metadata>,
}

impl<T> GenericResponse<T> {
    pub fn new(status: Status) -> Self {
        GenericResponse {
            status: status.code,
            message: status.reason().unwrap_or_default().to_string(),
            errors: None,
            data: None,
            timestamp: chrono::Local::now().naive_local(),
            metadata: None,
        }
    }

    pub fn ok(data: T) -> Self {
        GenericResponse {
            status: Status::Ok.code,
            message: Status::Ok.reason().unwrap_or_default().to_string(),
            errors: None,
            data: Some(data),
            timestamp: chrono::Local::now().naive_local(),
            metadata: None,
        }
    }

    pub fn add_error(mut self, error: Error) -> GenericResponse<T> {
        if self.errors.is_none() {
            self.errors = Some(vec![]);
        }

        if let Some(errors) = &mut self.errors {
            errors.push(error);
        }

        return self;
    }

    pub fn to_json(self) -> Json<GenericResponse<T>> {
        Json(self)
    }
}
