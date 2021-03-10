CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password TEXT NOT NULL,

    profile_image TEXT NOT NULL,

    online BOOLEAN DEFAULT false NOT NULL,
    last_online TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL
);
