CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE urls (
    id BIGSERIAL PRIMARY KEY,

    long_url TEXT NOT NULL,
    
    short_code VARCHAR(32) UNIQUE,
    
    custom_domain TEXT,

    expires_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE INDEX idx_urls_short_code ON urls (short_code);


CREATE INDEX idx_urls_domain_slug ON urls (custom_domain, short_code);


CREATE INDEX idx_urls_expires_at ON urls (expires_at);


CREATE OR REPLACE FUNCTION set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_set_timestamp
BEFORE UPDATE ON urls
FOR EACH ROW
EXECUTE FUNCTION set_timestamp();