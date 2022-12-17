-- Add up migration script here

-- CREATE SCHEMA  template;
-- CREATE TYPE user_status AS ENUM ('online','offline','invisibility');
-- CREATE TYPE user_role AS ENUM ('visitor','user','admin','superadmin');


CREATE TABLE users
(
    id        uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    username  varchar(64) NOT NULL,
    password  varchar(64) NOT NULL,
    create_at timestamptz NOT NULL,
    update_at timestamptz NOT NULL,
    status    varchar(64) NOT NULL DEFAULT 'offline',
    role      varchar(64)   NOT NULL DEFAULT 'user'
);

insert into users (username, password, create_at, update_at, status, role)
values ('user', 'admin', now(), now(), 'offline','user'),
       ('admin', 'admin', now(), now(), 'offline','admin'),
       ('root', 'admin', now(), now(), 'offline','superadmin'),
       ('visitor', 'admin', now(), now(), 'offline','visitor');
