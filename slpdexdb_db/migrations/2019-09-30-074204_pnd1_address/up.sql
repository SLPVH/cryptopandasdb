-- Your SQL goes here

ALTER TABLE pending_pnd1_tx
    ADD COLUMN "owner_address" BYTEA;

UPDATE pending_pnd1_tx SET "owner_address" = E'';

ALTER TABLE pending_pnd1_tx
    ALTER COLUMN "owner_address" SET NOT NULL;
