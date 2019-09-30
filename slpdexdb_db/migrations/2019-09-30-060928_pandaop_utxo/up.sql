-- Your SQL goes here

CREATE TABLE pandaop_utxo (
    "tx_hash"   BYTEA NOT NULL,
    "vout"      INT NOT NULL,
    PRIMARY KEY ("tx_hash", "vout")
);
