-- Your SQL goes here

CREATE TABLE users (
    id TEXT NOT NULL CONSTRAINT users_pk PRIMARY KEY,
    username TEXT NOT NULL CONSTRAINT users_pk_2 UNIQUE,
    password TEXT NOT NULL
)
