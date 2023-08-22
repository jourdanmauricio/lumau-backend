use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use serde_json::json;

use crate::{
    middlewares::jwt_auth::{self, is_admin},
    modules::admin_users::{
        models::{LoginUser, RegisterUser, TokenClaims},
        services,
    },
    AppState,
};

#[post("/auth/register")]
async fn register_user(
    req: HttpRequest,
    data: web::Data<AppState>,
    body: web::Json<RegisterUser>,
    _: jwt_auth::JwtMiddleware,
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

    let mut db_connection = data.db.get().expect("couldn't get db connection from pool");

    match services::insert_new_user(&mut db_connection, body).await {
        Ok(user) => {
            return HttpResponse::Created().json(user);
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e)),
    }
}

#[post("/auth/login")]
async fn login_user(body: web::Json<LoginUser>, data: web::Data<AppState>) -> HttpResponse {
    let mut db_connection = data.db.get().expect("couldn't get db connection from pool");

    match services::get_user_by_url(&mut db_connection, &body.url).await {
        Ok(user) => {
            let parsed_hash = PasswordHash::new(&user.password).unwrap();

            let is_valid = Argon2::default()
                .verify_password(body.password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true);

            if !is_valid {
                return HttpResponse::BadRequest()
                    .json(json!({"status": "fail", "message": "Invalid url or password"}));
            }

            let now = Utc::now();
            let iat = now.timestamp() as usize;
            let exp = (now + Duration::minutes(60)).timestamp() as usize;
            let claims: TokenClaims = TokenClaims {
                sub: user.user_id.to_string(),
                name: user.name.to_string(),
                role: user.role.to_string(),
                exp,
                iat,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
            )
            .unwrap();

            let cookie = Cookie::build("token", token.to_owned())
                .path("/")
                .max_age(ActixWebDuration::new(60 * 60, 0))
                .http_only(true)
                .finish();

            println!("is_valid {}; user: {:#?} ", is_valid, user);
            // return HttpResponse::Created().json(user);
            HttpResponse::Ok()
                .cookie(cookie)
                .json(json!({"status": "success", "token": token}))
        }
        Err(e) => {
            HttpResponse::NotFound().json(json!({"status": "fail", "message": format!("{}", e)}))
        } // .body(format!("Something went wrong: {}", e)),
    }
}

#[get("/auth/logout")]
async fn logout(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user)
        .service(login_user)
        .service(logout);
}
