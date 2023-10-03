-- Add migration script here
CREATE TABLE IF NOT EXISTS accounts
(
    account        TEXT PRIMARY KEY NOT NULL,

    nonce          INTEGER DEFAULT 0 NOT NULL,

    created_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at     TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
