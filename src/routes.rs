use crate::models::{Link, LinkJson, LinkNew};
use crate::AppState;

use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

/// Returns the home page of the site.
pub async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

/// Adds a link entry to the database if not already added.
pub async fn add_link(
    data: web::Data<AppState>,
    item: web::Json<LinkJson>,
) -> Result<HttpResponse, Error> {
    Ok(
        match web::block(move || add_single_link(data, item)).await {
            Ok(link) => HttpResponse::Created().json(link),
            _ => HttpResponse::from(HttpResponse::InternalServerError()),
        },
    )
}

/// Get all the links in the database.
pub async fn get_links(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(match get_all_links(data).await {
        Ok(links) => HttpResponse::Ok().json(links),
        _ => HttpResponse::from(HttpResponse::InternalServerError()),
    })
}

// TODO move to a method of the `Link`.
fn add_single_link(data: web::Data<AppState>, item: web::Json<LinkJson>) -> Link {
    use crate::schema::links::dsl::*;
    let mut db_connection = data.db.get().expect("couldn't get db connection from pool");

    match links
        .filter(link.eq(&item.link))
        .first::<Link>(&mut db_connection)
    {
        Ok(result) => result,
        Err(_) => {
            let new_link = LinkNew {
                link: &item.link,
                title: &item.title,
                date_created: &format!("{}", chrono::Local::now().naive_local()),
            };

            insert_into(links)
                .values(&new_link)
                .execute(&mut db_connection)
                .expect("Error saving new link");

            let result = links.order(id.desc()).first(&mut db_connection).unwrap();
            result
        }
    }
}

// TODO move to a method of the `Link`.
async fn get_all_links(data: web::Data<AppState>) -> Result<Vec<Link>, diesel::result::Error> {
    use crate::schema::links::dsl::*;

    let mut db_connection = data.db.get().expect("couldn't get db connection from pool");
    // let result = links.load::<Link>(&db_connection)?;
    let result = links.load::<Link>(&mut db_connection).unwrap();
    Ok(result)
}
