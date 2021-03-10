CREATE TABLE IF NOT EXISTS friends (
    user_one uuid NOT NULL REFERENCES users,
    user_two uuid NOT NULL REFERENCES users,
    since TIMESTAMP NOT NULL,
    is_request BOOLEAN DEFAULT false NOT NULL
);
