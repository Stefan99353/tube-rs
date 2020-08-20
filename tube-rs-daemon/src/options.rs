use crate::db_models::{DefaultOption, InsertDefaultOption, OptionGroup};
use crate::models::QueryIdNumber;
use crate::option_actions;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Get all OptionGroups
//
#[get("/api/options/all")]
pub async fn get_option_groups(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Vec<(OptionGroup, Vec<DefaultOption>)> =
        web::block(move || option_actions::get_option_groups(&conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(HttpResponse::Ok().json(response))
}

// Get Option Group by id
//
#[get("/api/options/id")]
pub async fn get_option_group_by_id(
    pool: web::Data<DbPool>,
    id: web::Query<QueryIdNumber>,
) -> Result<HttpResponse, Error> {
    let group_id = id.into_inner().id;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Option<OptionGroup> =
        web::block(move || option_actions::get_option_group_by_id(group_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(HttpResponse::Ok().json(response))
}

// Add a new OptionGroup
//
#[post("/api/options/add")]
pub async fn add_option_group(
    pool: web::Data<DbPool>,
    option_group: web::Json<(OptionGroup, Vec<DefaultOption>)>,
) -> Result<HttpResponse, Error> {
    let (option_group, options) = option_group.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Option<OptionGroup> =
        web::block(move || option_actions::add_option_group(option_group, options, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(HttpResponse::Ok().json(response))
}

// Update OptionGroup
//
#[put("/api/options/update")]
pub async fn update_option_group(
    pool: web::Data<DbPool>,
    option_group: web::Json<OptionGroup>,
) -> Result<HttpResponse, Error> {
    let option_group = option_group.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Option<OptionGroup> =
        web::block(move || option_actions::update_option_group(option_group, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(HttpResponse::Ok().json(response))
}

// Delete OptionGroup
//
#[delete("/api/options/update")]
pub async fn remove_option_group(
    pool: web::Data<DbPool>,
    id: web::Query<QueryIdNumber>,
) -> Result<HttpResponse, Error> {
    let group_id = id.into_inner().id;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: bool = web::block(move || option_actions::remove_option_group(group_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

// Get Default Options for GroupID
//
#[get("/api/options/group/id")]
pub async fn get_default_options(
    pool: web::Data<DbPool>,
    id: web::Query<QueryIdNumber>,
) -> Result<HttpResponse, Error> {
    let group_id = id.into_inner().id;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Option<Vec<DefaultOption>> =
        web::block(move || option_actions::get_default_options_by_id(group_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(HttpResponse::Ok().json(response))
}

// Add Default Option
//
#[post("/api/options/group/add")]
pub async fn add_default_option(
    pool: web::Data<DbPool>,
    default_option: web::Json<InsertDefaultOption>,
) -> Result<HttpResponse, Error> {
    let default_option = default_option.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || option_actions::add_default_option(default_option, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(true))
}

// Update Default Option
//
#[put("/api/options/group/update")]
pub async fn update_default_option(
    pool: web::Data<DbPool>,
    default_option: web::Json<DefaultOption>,
) -> Result<HttpResponse, Error> {
    let default_option = default_option.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || option_actions::update_default_option(default_option, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(true))
}

// Remove Default Option
//
#[delete("/api/options/group/delete")]
pub async fn remove_default_option(
    pool: web::Data<DbPool>,
    id: web::Query<QueryIdNumber>,
) -> Result<HttpResponse, Error> {
    let option_id = id.into_inner().id;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || option_actions::remove_default_option(option_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(true))
}
