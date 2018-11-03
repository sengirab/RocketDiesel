CREATE TABLE users (
    uuid UUID PRIMARY KEY NOT NULL Default gen_random_uuid(),
    first_name VARCHAR UNIQUE NOT NULL,
    last_name VARCHAR UNIQUE NOT NULL,
    password VARCHAR UNIQUE NOT NULL
);