-- Add up migration script here

-- CREATE SCHEMA  template;
CREATE TYPE user_status AS ENUM ('online','offline','invisibility');
CREATE TYPE user_role AS ENUM ('visitor','user','admin','superadmin');


CREATE TABLE users
(
    id        uuid PRIMARY KEY     DEFAULT uuid_generate_v4(),
    username  varchar(64) NOT NULL,
    password  varchar(64) NOT NULL,
    create_at timestamptz NOT NULL,
    update_at timestamptz NOT NULL,
    status    user_status NOT NULL DEFAULT 'offline',
    role      user_role   NOT NULL DEFAULT 'user'
);

-- CREATE OR REPLACE FUNCTION users_trigger() RETURNS TRIGGER AS
-- $$
-- BEGIN
--     IF TG_OP = 'INSERT' THEN
--         UPDATE users SET create_at = now();
--     ELSIF TG_OP = 'UPDATE' THEN
--         UPDATE users SET update_at = now();
--     END IF;
-- END
-- $$ LANGUAGE plpgsql;

-- CREATE TRIGGER users_trigger
--     AFTER INSERT OR UPDATE
--     ON users
--     FOR EACH ROW
-- EXECUTE PROCEDURE users_trigger();
