-- Your SQL goes here

CREATE TABLE users (
    uuid TEXT NOT NULL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
)
