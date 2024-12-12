CREATE TABLE card_sets (
    id varchar PRIMARY KEY,
    name varchar NOT NULL,
    count bigint NOT NULL,
    url varchar NOT NULL,
    created_at timestamp DEFAULT current_timestamp NOT NULL,
    updated_at timestamp DEFAULT current_timestamp NOT NULL
);

CREATE TABLE cards (
    id varchar PRIMARY KEY,
    card_set varchar NOT NULL REFERENCES card_sets(id),
    name varchar NOT NULL,
    url varchar NOT NULL,
    image varchar NOT NULL,
    created_at timestamp DEFAULT current_timestamp NOT NULL,
    updated_at timestamp DEFAULT current_timestamp NOT NULL
);

CREATE TABLE collections (
    id uuid PRIMARY KEY,
    user_id varchar NOT NULL,
    created_at timestamp DEFAULT current_timestamp NOT NULL,
    updated_at timestamp DEFAULT current_timestamp NOT NULL
);

CREATE TABLE collection_cards (
    card_id varchar NOT NULL REFERENCES cards(id),
    collection_id uuid NOT NULL REFERENCES collections(id),
    created_at timestamp DEFAULT current_timestamp NOT NULL,
    updated_at timestamp DEFAULT current_timestamp NOT NULL,
    PRIMARY KEY (card_id, collection_id)
);