use super::{models::User, services};
use crate::{
    middlewares::jwt_auth::{self, is_admin},
    modules::auth::models::FilteredUser,
    AppState,
};
use actix_web::{error, get, web, HttpMessage, HttpRequest, HttpResponse, Responder};

use serde_json::json;

#[derive(Debug)]
struct NotAdmin;

#[get("/users")]
/// Get all the users in the database.
pub async fn get_users(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
    // ) -> Result<HttpResponse, Error> {
) -> HttpResponse {
    let ext = req.extensions();
    let role = ext.get::<String>().unwrap();

    match is_admin(role) {
        Ok(_) => {}
        _ => {
            return HttpResponse::Unauthorized()
                .json(json!({"status": "fail", "message": "UnAuthorized"}))
        }
    };

    match services::get_all_users(data).await {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::from(HttpResponse::InternalServerError()),
    }
}

#[get("/users/{id}")]
/// Get user in the database.
async fn get_user(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let id = id.into_inner();

    let user = web::block(move || {
        let mut db_connection = data.db.get().expect("couldn't get db connection from pool");
        services::get_user(&mut db_connection, &id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body(format!("No user found with ID: {id}")),
    })
}

#[get("/users/me")]
async fn get_me(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: jwt_auth::JwtMiddleware,
) -> actix_web::Result<impl Responder> {
    let ext = req.extensions();
    let user_id = ext.get::<String>().unwrap();

    let user_id = user_id.parse::<i32>().unwrap();

    let user = web::block(move || {
        let mut db_connection = data.db.get().expect("couldn't get db connection from pool");
        services::get_user(&mut db_connection, &user_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        Some(user) => {
            let json_response = serde_json::json!({
                "status":  "success",
                "user": filter_user_record(&user),
            });

            HttpResponse::Ok().json(json_response)
        }
        None => HttpResponse::NotFound().body(format!("No user found with ID: ")),
    })
}

fn filter_user_record(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.user_id.to_string(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        role: user.role.to_owned(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users).service(get_me).service(get_user);
}
