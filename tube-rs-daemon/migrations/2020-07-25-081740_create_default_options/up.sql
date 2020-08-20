-- Your SQL goes here

CREATE TABLE option_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE default_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    flag TEXT NOT NULL,
    val TEXT,
    group_id INTEGER NOT NULL REFERENCES option_groups(id)
);