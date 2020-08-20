-- Your SQL goes here

CREATE TABLE videos (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    added_at TEXT NOT NULL,
    downloaded BOOLEAN NOT NULL DEFAULT 0,
    path TEXT NOT NULL,
    quality TEXT NOT NULL
);

CREATE TABLE video_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    flag TEXT NOT NULL NOT NULL,
    val TEXT,
    video_id TEXT NOT NULL REFERENCES videos(id)
);