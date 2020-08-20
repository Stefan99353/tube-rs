use diesel::prelude::*;
use std::process::Command;

use crate::db_models::{InsertVideoOption, Video, VideoOption};

pub fn add_video(
    new_video: Video,
    options: Vec<VideoOption>,
    conn: &SqliteConnection,
) -> Result<Option<Video>, diesel::result::Error> {
    use crate::schema::{video_options, videos};

    // Check if Video exists
    let result: Option<Video> = videos::table
        .filter(videos::id.eq(new_video.id.to_string()))
        .first::<Video>(conn)
        .optional()?;

    match result {
        Some(_) => {
            // Already in DB
            Ok(None)
        }
        None => {
            let mut ins_options: Vec<InsertVideoOption> = vec![];

            for option in options {
                ins_options.push(InsertVideoOption {
                    flag: option.flag,
                    val: option.val,
                    video_id: new_video.id.to_string(),
                })
            }

            diesel::insert_into(videos::table)
                .values(&new_video)
                .execute(conn)?;

            diesel::insert_into(video_options::table)
                .values(&ins_options)
                .execute(conn)?;

            Ok(Some(new_video))
        }
    }
}

pub fn update_video(
    video: Video,
    conn: &SqliteConnection,
) -> Result<Option<Video>, diesel::result::Error> {
    use crate::schema::videos;

    let result: Option<Video> = videos::table
        .filter(videos::id.eq(video.id.to_string()))
        .first::<Video>(conn)
        .optional()?;

    match result {
        None => Ok(None),
        Some(old_video) => {
            diesel::update(videos::table.find(old_video.id))
                .set((
                    videos::title.eq(video.title.to_string()),
                    videos::added_at.eq(video.added_at.to_string()),
                    videos::downloaded.eq(video.downloaded),
                    videos::path.eq(video.path.to_string()),
                    videos::quality.eq(video.quality.to_string()),
                ))
                .execute(conn)?;

            Ok(Some(video))
        }
    }
}

pub fn remove_video(
    video_id: String,
    conn: &SqliteConnection,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::{video_options, videos};

    let result: Option<Video> = videos::table
        .filter(videos::id.eq(video_id.to_string()))
        .first::<Video>(conn)
        .optional()?;

    match result {
        None => Ok(false),
        Some(video) => {
            diesel::delete(
                video_options::table.filter(video_options::video_id.eq(video.id.to_string())),
            )
            .execute(conn)?;

            diesel::delete(videos::table.filter(videos::id.eq(video.id))).execute(conn)?;

            Ok(true)
        }
    }
}

pub fn get_video_info(video_id: String) -> Result<Option<Video>, anyhow::Error> {
    let output = Command::new("youtube-dl")
        .arg(&video_id)
        .arg("--get-title")
        .output()?;

    let title = String::from_utf8_lossy(&output.stdout);
    let title = title.trim_matches(char::from(10));

    let video = Video {
        id: video_id,
        title: title.to_string(),
        added_at: "".to_string(),
        downloaded: false,
        path: "".to_string(),
        quality: "".to_string(),
    };

    Ok(Some(video))
}

// Run query using Diesel to find a track by its ID
pub fn get_video_by_id(
    video_id: String,
    conn: &SqliteConnection,
) -> Result<Option<Video>, diesel::result::Error> {
    use crate::schema::videos::dsl::*;

    let result = videos
        .filter(id.eq(video_id))
        .first::<Video>(conn)
        .optional()?;

    Ok(result)
}

// Run query using Diesel to get all tracks
pub fn get_queued_videos(
    down: bool,
    conn: &SqliteConnection,
) -> Result<Vec<Video>, diesel::result::Error> {
    use crate::schema::videos::dsl::*;

    let result = videos.filter(downloaded.eq(down)).load::<Video>(conn)?;

    Ok(result)
}

pub fn get_videos(conn: &SqliteConnection) -> Result<Vec<Video>, diesel::result::Error> {
    use crate::schema::videos::dsl::*;

    let result = videos.load::<Video>(conn)?;

    Ok(result)
}
