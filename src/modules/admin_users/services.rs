use crate::modules::admin_users::models::User;
use crate::AppState;
use actix_web::web;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use log::info;
use rand_core::OsRng;
use thiserror::Error;

use super::models::RegisterUser;
type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Error, Debug)]
pub enum RepositoryError {
    // #[error("PoisonError: `{0}`")]
    // LockError(String),
    #[error("This entity already exists")]
    AlreadyExists,
    #[error("This entity does not exists")]
    DoesNotExists,
    // #[error("The id format is not valid")]
    // InvalidId,
}

type RepositoryResult<T> = Result<T, RepositoryError>;

pub async fn get_all_users(data: web::Data<AppState>) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::admin_users::dsl::*;

    let mut db_connection = data.db.get().expect("couldn't get db connection from pool");

    let result = admin_users.load::<User>(&mut db_connection).unwrap();
    Ok(result)
}

pub fn get_user(conn: &mut SqliteConnection, uid: &i32) -> Result<Option<User>, DbError> {
    use crate::schema::admin_users::dsl::*;

    let user = admin_users
        .filter(user_id.eq(uid))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub async fn get_user_by_url(conn: &mut SqliteConnection, uurl: &String) -> RepositoryResult<User> {
    use crate::schema::admin_users::dsl::*;

    let result = admin_users.filter(url.eq(uurl)).first::<User>(conn);

    result.map_err(|e| {
        info!("Service Error{:?}", e);
        RepositoryError::DoesNotExists
    })
}

/// Run query using Diesel to insert a new database row and return the result.
pub async fn insert_new_user(
    conn: &mut SqliteConnection,
    body: web::Json<RegisterUser>,
) -> RepositoryResult<User> {
    use crate::schema::admin_users::dsl::*;

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let new_user = RegisterUser {
        name: body.name.to_owned(),
        url: body.url.to_owned(),
        front_deploy: body.front_deploy.to_owned(),
        email: body.email.to_owned(),
        // password: body.password.to_owned(),
        password: hashed_password.to_owned(),
        phone: body.phone.to_owned(),
        dni: body.dni.to_owned(),
        status: body.status.to_owned(),
        role: body.role.to_owned(),
        created_at: format!("{}", chrono::Local::now().naive_local()),
        updated_at: format!("{}", chrono::Local::now().naive_local()),
    };

    let result = diesel::insert_into(admin_users)
        .values(&new_user)
        //.execute(conn);
        .get_result::<User>(conn);

    result.map_err(|e| {
        info!("{:?}", e);
        RepositoryError::AlreadyExists
    })
}
