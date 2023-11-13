-- Your SQL goes here
CREATE TABLE USERS (
    id SERIAL PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL,
    salt VARCHAR(64) NOT NULL,
    hashed_password VARCHAR(64) NOT NULL,
    first_name TEXT NOT NULL
)