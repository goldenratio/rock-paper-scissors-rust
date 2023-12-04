use crate::AppState;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::HeaderValue;
use actix_web::{web, Error as ActixWebError, FromRequest, HttpRequest};
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
pub struct AdminClaims {
    pub id: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminAuthentication;

impl FromRequest for AdminAuthentication {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        let app_state = &req.app_data::<web::Data<AppState>>().unwrap();
        let admin_info = app_state.admin_info.lock().unwrap();

        let authorization_header_option: Option<&HeaderValue> =
            req.headers().get(actix_web::http::header::AUTHORIZATION);
        // No Header was sent
        if authorization_header_option.is_none() {
            return ready(Err(ErrorUnauthorized("No authentication token sent!")));
        }

        let authentication_token: String = authorization_header_option
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string();
        // Couldn't convert Header::Authorization to String
        if authentication_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid authentication token sent!")));
        }
        let client_auth_token = authentication_token[6..authentication_token.len()].trim();
        let admin_jwt_secret: &str = admin_info.admin_jwt_secret.as_str();

        let token_result: Result<TokenData<AdminClaims>, JwtError> = decode::<AdminClaims>(
            client_auth_token,
            &DecodingKey::from_secret(admin_jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        match token_result {
            Ok(token) => {
                println!("{:}", token.claims.id);
                ready(Ok(AdminAuthentication {}))
            }
            Err(e) => {
                println!("token_result Error: {:}", e);
                ready(Err(ErrorUnauthorized("Invalid authentication token sent!")))
            }
        }
    }
}
