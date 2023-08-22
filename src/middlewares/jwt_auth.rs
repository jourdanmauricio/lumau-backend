use core::fmt;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

// use crate::model::TokenClaims;
use crate::modules::admin_users::models::TokenClaims;
use crate::AppState;
//use crate::modules::admin_users::models::TokenClaims;
// use crate::config::types::AppState;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct JwtMiddleware {
    pub user_id: String,
    pub name: String,
    pub role: String,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        // let user_id = claims.sub.user_id.as_str();

        let user_id = claims.sub.as_str().to_string();
        let name = claims.name.as_str().to_string();
        let role = claims.role.as_str().to_string();
        req.extensions_mut().insert::<String>(user_id.to_owned());
        req.extensions_mut().insert::<String>(name.to_owned());
        req.extensions_mut().insert::<String>(role.to_owned());

        ready(Ok(JwtMiddleware {
            user_id,
            name,
            role,
        }))
    }
}

pub fn is_admin(role: &String) -> Result<&String, String> {
    if role == "admin" {
        Ok(role)
    } else {
        Err(format!("'{}' is not long enough!", role))
    }
}
