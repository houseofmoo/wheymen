use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header, web::Data, Error, FromRequest,
    HttpRequest,
};
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{self, decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtClaims {
    pub aud: String,
    pub exp: i32,
    pub sub: String,
    pub email: String,
    pub phone: String,
    pub role: String,
    pub session_id: String,
}

#[derive(Clone)]
pub struct JwtDecoder {
    pub key: DecodingKey,
    pub config: Validation,
}

impl JwtDecoder {
    pub fn new(key: String) -> JwtDecoder {
        JwtDecoder {
            key: DecodingKey::from_secret(key.as_bytes()),
            config: Validation::new(Algorithm::HS256),
        }
    }

    pub fn decode_jwt(&self, token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        decode::<JwtClaims>(token, &self.key, &self.config)
            .map(|data| data.claims)
            .map_err(|e| e)
    }
}

#[derive(Debug, Deserialize)]
pub struct Authorized {
    pub user_id: String,
    pub user_email: String,
}

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.app_data::<Data<JwtDecoder>>() {
            Some(decoder) => match req.headers().get(header::AUTHORIZATION) {
                Some(header) => match header.to_str() {
                    Ok(token_str) => match decoder.decode_jwt(token_str) {
                        Ok(claims) => ok(Authorized {
                            user_id: claims.sub,
                            user_email: claims.email,
                        }),
                        Err(_) => err(ErrorUnauthorized("unauthorized user")),
                    },
                    Err(_) => err(ErrorUnauthorized("auth invalid format")),
                },
                None => err(ErrorUnauthorized("missing auth")),
            },
            None => err(ErrorUnauthorized("auth decoder unavailable")),
        }
    }
}
