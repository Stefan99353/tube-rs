-- Your SQL goes here

CREATE TABLE youtube_flags (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    flag TEXT NOT NULL
);

CREATE TABLE flag_values (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    value TEXT NOT NULL,
    youtube_flag_id INTEGER NOT NULL REFERENCES youtube_flags(id)
);