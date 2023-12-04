use std::process::id;
use crate::AppState;
use actix_web::{web, Responder, post};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::error_enums::AdminError;
use crate::extractors::admin_auth_token::AdminClaims;

#[derive(Deserialize, Debug)]
struct AdminStatusRequestData {
    #[serde(rename = "username")]
    username: String,
    #[serde(rename = "password")]
    password: String
}

#[derive(Serialize)]
enum AdminStatusResponseData {
    #[serde(rename = "data")]
    Success {
        #[serde(rename = "jwtToken")]
        jwt_token: String,
    },
    #[serde(rename = "data")]
    Error {
        #[serde(rename = "error")]
        error_type: AdminError
    },
}

#[post("/admin_status")]
async fn admin_status(param_obj: web::Json<AdminStatusRequestData>, state: web::Data<AppState>) -> impl Responder {
    let payload = param_obj.into_inner();
    println!("/admin_status {:?}", payload);

    if validate_admin_credentials(&payload).is_err() {
        return web::Json(AdminStatusResponseData::Error {
            error_type: AdminError::GenericError
        });
    }

    let admin_info = state.admin_info.lock().unwrap();
    let token_expiry_date = (Utc::now() + Duration::minutes(18)).timestamp() as usize;
    let claims = AdminClaims {
        id: 9,
        exp: token_expiry_date
    };

    let jwt_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(admin_info.admin_jwt_secret.as_str().as_ref())
    ).unwrap();

    let response_data = AdminStatusResponseData::Success { jwt_token };
    return web::Json(response_data);
}

fn validate_admin_credentials(data: &AdminStatusRequestData) -> Result<(), AdminError> {
    // TODO: validate admin username and pwd
    Ok(())
}
