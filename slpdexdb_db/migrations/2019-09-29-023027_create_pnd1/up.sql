-- Your SQL goes here

CREATE TABLE pending_pnd1_tx (
    "tx"        BIGINT NOT NULL PRIMARY KEY REFERENCES tx (id) ON DELETE CASCADE,
    "father"    BIGINT NOT NULL REFERENCES panda (id) ON DELETE CASCADE,
    "mother"    BIGINT NOT NULL REFERENCES panda (id) ON DELETE CASCADE,
    "name"      TEXT NOT NULL
);
