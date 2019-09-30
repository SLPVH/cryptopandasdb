-- This file should undo anything in `up.sql`

ALTER TABLE token
    DROP COLUMN "parent_token",
    DROP COLUMN "parent_token_hash";
