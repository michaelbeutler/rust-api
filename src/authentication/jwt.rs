use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

fn is_valid(token: &str) -> Result<TokenData<Jwt>, jsonwebtoken::errors::Error> {
    // Strip the Bearer prefix from the token
    let token = token.trim_start_matches("Bearer ");
    let key = b"secret";
    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some("b@b.com".to_string());
    validation.set_audience(&["me"]);

    return decode::<Jwt>(&token, &DecodingKey::from_secret(key), &validation);
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Jwt {
    type Error = ErrorKind;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Failure((
                rocket::http::Status::Unauthorized,
                ErrorKind::InvalidKeyFormat,
            )),
            Some(key) => match is_valid(key) {
                Ok(c) => Outcome::Success(c.claims),
                Err(e) => Outcome::Failure((rocket::http::Status::Unauthorized, e.kind().clone())),
            },
        }
    }
}
