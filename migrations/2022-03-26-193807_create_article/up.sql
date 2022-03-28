CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TABLE article (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE article ADD blogger_id integer default 1
CONSTRAINT blogger_id REFERENCES blogger NOT NULL;


CREATE TRIGGER set_article_timestamp
BEFORE UPDATE ON article
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

