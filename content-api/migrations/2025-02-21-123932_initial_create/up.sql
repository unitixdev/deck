CREATE TABLE users (
    id          SERIAL      NOT NULL PRIMARY KEY,
    public_id   UUID        NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP,
    deleted_at  TIMESTAMP
);

CREATE TABLE sets (
    id          TEXT        NOT NULL PRIMARY KEY,
    name        TEXT        NOT NULL,
    count       INT         NOT NULL,
    total       INT         NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP,
    deleted_at  TIMESTAMP
);

CREATE TABLE cards (
    id          TEXT PRIMARY KEY            NOT NULL,
    set_id      TEXT REFERENCES sets(id)    NOT NULL,
    name        TEXT                        NOT NULL,
    url         TEXT                        NOT NULL,
    created_at  TIMESTAMP                 NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP,
    deleted_at  TIMESTAMP
);

CREATE TABLE user_cards (
    user_id     SERIAL      NOT NULL REFERENCES users(id),
    card_id     TEXT        NOT NULL REFERENCES cards(id),
    amount      INT         NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP,
    deleted_at  TIMESTAMP,

    PRIMARY KEY (user_id, card_id)
);
