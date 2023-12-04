use crate::app_state::{AdminInfo, AppState};
use crate::error_enums::AdminError;
use crate::extractors::admin_auth_token::AdminClaims;
use actix_web::{post, web, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct AdminStatusRequestData {
    #[serde(rename = "username")]
    username: String,
    #[serde(rename = "password")]
    password: String,
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
        error_type: AdminError,
    },
}

#[post("/admin_status")]
async fn admin_status(
    param_obj: web::Json<AdminStatusRequestData>,
    state: web::Data<AppState>,
) -> impl Responder {
    let payload = param_obj.into_inner();
    println!("/admin_status {:?}", payload);

    let admin_info = state.admin_info.lock().unwrap();
    if validate_admin_credentials(&payload, &admin_info).is_err() {
        return web::Json(AdminStatusResponseData::Error {
            error_type: AdminError::GenericError,
        });
    }

    let claims = AdminClaims::new();

    let jwt_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(admin_info.admin_jwt_secret.as_str().as_ref()),
    )
    .unwrap();

    let response_data = AdminStatusResponseData::Success { jwt_token };
    return web::Json(response_data);
}

fn validate_admin_credentials(
    client_data: &AdminStatusRequestData,
    admin_info: &AdminInfo,
) -> Result<(), AdminError> {
    if client_data.username == admin_info.admin_username
        && client_data.password == admin_info.admin_password
    {
        return Ok(());
    }
    return Err(AdminError::GenericError);
}
