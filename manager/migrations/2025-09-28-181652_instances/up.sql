-- Your SQL goes here

CREATE TABLE instances
(
    uuid             INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    name             TEXT    NOT NULL,
    factorio_version TEXT    NOT NULL
)
