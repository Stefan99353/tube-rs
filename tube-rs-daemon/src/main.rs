#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod daemon;
mod db_models;
mod models;
mod option_actions;
mod options;
mod schema;
mod video_actions;
mod videos;

use actix_cors::Cors;
use actix_files;
use actix_web::{middleware, web, App, HttpServer, Result};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=debug");
    env_logger::init();
    //dotenv::dotenv().ok();

    // Setup Database Connection
    let db_url = match std::env::var("TUBERS_DATABASE_URL") {
        Ok(db) => db,
        Err(_) => "tubers.sqlite".to_string(),
    };
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Run migrations
    embed_migrations!();
    let conn = pool
        .clone()
        .get()
        .expect("Couldn't get db connection from pool");
    embedded_migrations::run(&conn).expect("Unable to run migrations.");

    // Daemon
    let _daemon = daemon::TubeRsDaemon::start(pool.clone());

    // Http Server
    let mut path = std::env::current_exe()?;
    path.pop();
    //let path = path.to_str()?;

    let bind = "0.0.0.0:8080";
    println!("Starting server at: {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(Cors::new().supports_credentials().finish())
            .service(web::resource("/").route(web::get().to(index)))
            .service(actix_files::Files::new(
                "/static",
                format!("{}", path.display()),
            ))
            .service(videos::get_videos)
            .service(videos::get_queued_videos)
            .service(videos::get_finished_videos)
            .service(videos::get_video_by_id)
            .service(videos::get_video_info)
            .service(videos::add_video)
            .service(videos::update_video)
            .service(videos::remove_video)
            .service(options::get_option_groups)
            .service(options::get_option_group_by_id)
            .service(options::add_option_group)
            .service(options::update_option_group)
            .service(options::remove_option_group)
            .service(options::get_default_options)
            .service(options::add_default_option)
            .service(options::update_default_option)
            .service(options::remove_default_option)
    })
    .bind(&bind)?
    .run()
    .await
}

async fn index() -> Result<actix_files::NamedFile> {
    let mut path = std::env::current_exe()?;
    path.pop();
    Ok(actix_files::NamedFile::open(format!(
        "{}/index.html",
        path.display()
    ))?)
}
