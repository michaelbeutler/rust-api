use crate::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn should_handle_all_errors() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    let codes = vec![400, 401, 403, 404, 415, 416, 418, 500, 501, 502, 503, 504];
    for code in codes {
        let response = client.get(format!("/errors/{}", code)).dispatch();
        assert_eq!(response.status(), Status::from_code(code).unwrap());

        // Check body
        let body = response.into_string().unwrap();
        assert!(body.contains(&code.to_string()));
    }
}
