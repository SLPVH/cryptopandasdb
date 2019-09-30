ALTER TABLE token
    ADD COLUMN "parent_token" INT REFERENCES token (id) ON DELETE CASCADE,
    ADD COLUMN "parent_token_hash" BYTEA REFERENCES token (hash) ON DELETE CASCADE;
