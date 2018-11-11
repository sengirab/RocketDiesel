CREATE TABLE users (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    username VARCHAR UNIQUE NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    password VARCHAR UNIQUE NOT NULL
);