use dotenvy::dotenv;
use jsonwebtoken as jwt;
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{FromRequest, Request},
    serde::{Deserialize, DeserializeOwned, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub iat: u128,
    // 3 -> Principal
    // 2 -> Teacher
    // 1 -> Student
    // 0 -> Everyone Else
    pub id: u8,
    pub aud: String,
    pub sub: String,
    pub exp: u128,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, (Self::Error, Self::Error), ()> {
        let header = match req.headers().get_one("Authorization") {
            Some(header) => header,
            None => return Outcome::Failure((Status::Unauthorized, Status::Unauthorized)),
        };

        let fail_header = "this_is_a_legit_jwt_trust_me_bro";

        let result = if let Some(header) = header.strip_prefix("Bearer") {
            decode_jwt::<Claims>(header, &DECODE_KEY)
        } else {
            jwt::decode(fail_header, &DECODE_KEY, &jwt::Validation::default())
                .map_err(|_| Status::Unauthorized)
                .map(|data| data.claims)
        };

        match result {
            Ok(user) => Outcome::Success(user),
            Err(status) => Outcome::Failure((status, status)),
        }
    }
}

lazy_static::lazy_static! {
    static ref ENCODE_KEY: jwt::EncodingKey = {
        dotenv().ok();

        let secret = std::env::var("JWT_SECRET").expect("`JWT_SECRET` must be set");

        jwt::EncodingKey::from_secret(secret.as_bytes())
    };

    static ref DECODE_KEY: jwt::DecodingKey = {
        dotenv().ok();

        let secret = std::env::var("JWT_SECRET").expect("`JWT_SECRET` must be set");

        jwt::DecodingKey::from_secret(secret.as_bytes())
    };
}

fn _encode_jwt(user: Claims) -> Result<String, jwt::errors::Error> {
    let user_access = Claims {
        iat: user.iat,
        id: user.id,
        aud: user.aud,
        sub: user.sub,
        exp: user.exp,
    };

    jwt::encode(&jwt::Header::default(), &user_access, &ENCODE_KEY)
}

fn decode_jwt<T: DeserializeOwned>(
    jwtoken: &str,
    jwt_secret: &jwt::DecodingKey,
) -> Result<Claims, Status> {
    jwt::decode(jwtoken, &jwt_secret, &jwt::Validation::default())
        .map_err(|_| Status::Unauthorized)
        .map(|data| data.claims)
}
