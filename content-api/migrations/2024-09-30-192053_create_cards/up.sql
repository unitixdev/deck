CREATE TABLE IF NOT EXISTS cards(
    id          TEXT    NOT NULL,
    expansion   TEXT    NOT NULL,
    name        TEXT    NOT NULL,

    PRIMARY KEY(id, expansion)
);

CREATE TABLE IF NOT EXISTS users(
    id             TEXT     NOT NULL    PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS collections(
    id      SERIAL  NOT NULL    PRIMARY KEY,
    user_id TEXT    REFERENCES users(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS collection_cards(
    card_id         TEXT        NOT NULL,
    card_expansion  TEXT        NOT NULL,
    collections_id  SERIAL      REFERENCES collections(id) NOT NULL,
    amount          SMALLINT    NOT NULL,
    rating          SMALLINT    NOT NULL,
    created_at      TIMESTAMPTZ DEFAULT NOW(),

    PRIMARY KEY (card_id, card_expansion, collections_id),
    FOREIGN KEY (card_id, card_expansion) REFERENCES cards(id, expansion)
);
