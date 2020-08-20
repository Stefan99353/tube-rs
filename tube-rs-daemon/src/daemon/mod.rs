use chrono::{DateTime, Local};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use std::process::Command;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime};

use crate::db_models::{Video, VideoOption};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct TubeRsDaemon {}

impl TubeRsDaemon {
    pub fn start(pool: DbPool) -> JoinHandle<()> {
        //let pool = pool.clone();
        thread::spawn(move || {
            println!("Start Daemon loop");
            loop {
                // Check if in timerange
                let now = SystemTime::now();
                let datetime: DateTime<Local> = now.into();
                let time_string = datetime.format("%H:%M").to_string();
                let current_minutes = time_string_to_minutes(time_string);

                let start_minutes =
                    time_string_to_minutes(match std::env::var("TUBERS_START_TIME") {
                        Ok(sm) => sm,
                        Err(_) => "23:00".to_string(),
                    });
                let end_minutes = time_string_to_minutes(match std::env::var("TUBERS_END_TIME") {
                    Ok(em) => em,
                    Err(_) => "05:00".to_string(),
                });

                println!(
                    "Now: {}; Start: {}; End: {}",
                    &current_minutes, &start_minutes, &end_minutes
                );

                let mut in_time_range: bool = false;

                if (start_minutes < end_minutes)
                    && (current_minutes >= start_minutes && current_minutes < end_minutes)
                {
                    in_time_range = true;
                } else if (start_minutes <= current_minutes && current_minutes < 1439)
                    || (current_minutes < end_minutes)
                {
                    in_time_range = true;
                } else {
                    println!("Not in time range! Waiting 300 secs");

                    thread::sleep(Duration::from_secs(300));
                }

                if in_time_range {
                    download_video(&pool);
                }
            }
        })
    }
}

pub fn download_video(pool: &DbPool) -> Option<Video> {
    use crate::schema::videos;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    println!("Fetch for tracks");

    let result: Option<Video> = videos::table
        .filter(videos::downloaded.eq(false))
        .first::<Video>(&conn)
        .optional()
        .expect("Error getting tracks from db");

    let video = match result {
        None => {
            println!("No videos available to download!");

            thread::sleep(Duration::from_secs(300));
            None
        }
        Some(video) => {
            use crate::schema::video_options;
            // Get Options
            let options: Vec<VideoOption> = video_options::table
                .filter(video_options::video_id.eq(&video.id))
                .load::<VideoOption>(&conn)
                .expect("Error loading options");

            // Download Video

            // youtube-dl.exe --newline -i --restrict-filenames -o "C:\Users\srupe\Desktop\%(title)s.%(ext)s" -x --audio-format mp3 --audio-quality 0 --ignore-config --hls-prefer-native "l2UktJtKpFw"
            let mut command = Command::new("youtube-dl");

            // Download rate
            match std::env::var("TUBERS_START_TIME") {
                Ok(rate) => {
                    command.arg("--limit-rate").arg(rate);
                }
                Err(_) => {}
            }

            // Download dir
            match std::env::var("TUBERS_DATA") {
                Ok(dir) => {
                    command
                        .arg("--output")
                        .arg(format!("{}/%(title)s.%(ext)s", dir));
                }
                Err(_) => {}
            }

            // Options
            for option in options {
                match option.val {
                    None => {
                        command.arg(option.flag);
                    }
                    Some(val) => {
                        command.arg(option.flag).arg(val);
                    }
                }
            }

            // Track ID
            command.arg(&video.id).arg("--quiet");

            // @TODO Result handling
            command.output().unwrap();

            // Update DB
            diesel::update(videos::table.find(&video.id))
                .set(videos::downloaded.eq(true))
                .execute(&conn)
                .expect("Unable to find track in DB");

            Some(video)
        }
    };

    video
}

pub fn time_string_to_minutes(time: String) -> u32 {
    let split = time.split(":");
    let split: Vec<&str> = split.collect();
    let hours: u32 = match split[0].parse::<u32>() {
        Ok(h) => h,
        Err(_) => 23,
    };
    let minutes: u32 = match split[1].parse::<u32>() {
        Ok(m) => m,
        Err(_) => 0,
    };

    hours * 60 + minutes
}
