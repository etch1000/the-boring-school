use dotenvy::dotenv;
use jsonwebtoken as jwt;
use okapi::openapi3::{
    MediaType, Object, Response as OpenApiResponse, Responses, SecurityRequirement, SecurityScheme,
    SecuritySchemeData,
};
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{FromRequest, Request},
    serde::{Deserialize, DeserializeOwned, Serialize},
};
use rocket_okapi::{gen::OpenApiGenerator, request::OpenApiFromRequest};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

        let result = if let Some(header) = header.strip_prefix("Bearer ") {
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

impl<'r> OpenApiFromRequest<'r> for Claims {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requires a Bearer Token to Access".to_owned()),
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };

        let mut security_req = SecurityRequirement::new();

        security_req.insert("Claims".to_owned(), Vec::new());

        Ok(rocket_okapi::request::RequestHeaderInput::Security(
            "Claims".to_owned(),
            security_scheme,
            security_req,
        ))
    }

    fn get_responses(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
    ) -> rocket_okapi::Result<Responses> {
        use okapi::openapi3::RefOr;

        Ok(Responses {
            responses: okapi::map! {"401".to_owned() => RefOr::Object(unauthorized_response(_gen))},
            ..Default::default()
        })
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

fn unauthorized_response(gen: &mut OpenApiGenerator) -> OpenApiResponse {
    let schema = gen.json_schema::<Claims>();

    OpenApiResponse {
        description: "\
            # [401 Unauthorized](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)\n\
            This response is given when you request a page that you don't have access to \
            or you have not provided any authentication. "
            .to_owned(),
        content: okapi::map! {
            "application/json".to_owned() => MediaType {
                schema: Some(schema),
                ..Default::default()
            }
        },
        ..Default::default()
    }
}
