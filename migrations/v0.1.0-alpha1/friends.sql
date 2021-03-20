CREATE TYPE friends_state AS ENUM
(
    'requested',
    'approved'
);

CREATE TABLE IF NOT EXISTS friends
(
    transmitter uuid          NOT NULL,
    receiver    uuid          NOT NULL,
    state       friends_state NOT NULL,
    since       TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP
);
