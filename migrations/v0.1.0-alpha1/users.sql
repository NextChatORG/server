CREATE TABLE IF NOT EXISTS users
(
    id              uuid        NOT NULL DEFAULT uuid_generate_v4 () PRIMARY KEY,
    username        VARCHAR(15) NOT NULL UNIQUE,
    password        TEXT        NOT NULL,

    profile_image   TEXT        NOT NULL,

    logged          BOOLEAN     NOT NULL DEFAULT false,
    last_online     TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at      TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);
