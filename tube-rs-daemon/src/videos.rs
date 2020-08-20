use crate::db_models::{Video, VideoOption};
use crate::models::QueryIdString;
use crate::video_actions;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Get all videos
//
#[get("/api/videos/all")]
pub async fn get_videos(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Vec<Video> = web::block(move || video_actions::get_videos(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

// Get queued videos
//
#[get("/api/videos/queued")]
pub async fn get_queued_videos(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Vec<Video> = web::block(move || video_actions::get_queued_videos(false, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

// Get finished tracks
//
#[get("/api/videos/finished")]
pub async fn get_finished_videos(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Vec<Video> = web::block(move || video_actions::get_queued_videos(true, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

// Get track by ID
//
#[get("/api/videos/id")]
pub async fn get_video_by_id(
    pool: web::Data<DbPool>,
    id: web::Query<QueryIdString>,
) -> Result<HttpResponse, Error> {
    let video_id = id.into_inner().id;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let video: Option<Video> = web::block(move || video_actions::get_video_by_id(video_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(video))
}

// Get Video Information by ID from YouTube
//
#[get("/api/videos/info")]
pub async fn get_video_info(id: web::Query<QueryIdString>) -> Result<HttpResponse, Error> {
    let video_id = id.into_inner().id;

    // use web::block to offload blocking code without blocking server thread
    let response: Option<Video> = web::block(move || video_actions::get_video_info(video_id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);

            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

// Add a new track
//
#[post("/api/videos/add")]
pub async fn add_video(
    pool: web::Data<DbPool>,
    video: web::Json<(Video, Vec<VideoOption>)>,
) -> Result<HttpResponse, Error> {
    let (video, options) = video.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: Option<Video> =
        web::block(move || video_actions::add_video(video, options, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(HttpResponse::Ok().json(response))
}

// Update a track
//
#[put("/api/videos/update")]
pub async fn update_video(
    pool: web::Data<DbPool>,
    video: web::Json<Video>,
) -> Result<HttpResponse, Error> {
    let video = video.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let response: Option<Video> = web::block(move || video_actions::update_video(video, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

// Remove a track
//
#[delete("/api/videos/delete")]
pub async fn remove_video(
    pool: web::Data<DbPool>,
    id: web::Query<QueryIdString>,
) -> Result<HttpResponse, Error> {
    let video_id = id.into_inner().id;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let response: bool = web::block(move || video_actions::remove_video(video_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}
