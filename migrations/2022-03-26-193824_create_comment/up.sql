CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = TO_CHAR(NOW(),'HH24:MI:SS');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TABLE comment (
    id SERIAL PRIMARY KEY,
    content VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL DEFAULT TO_CHAR(NOW(), 'HH24:MI:SS'),
    updated_at VARCHAR NOT NULL DEFAULT TO_CHAR(NOW(), 'HH24:MI:SS'),
    unique_id VARCHAR NOT NULL
);


ALTER TABLE comment ADD blogger_id integer default 1
CONSTRAINT blogger_id REFERENCES blogger NOT NULL;

ALTER TABLE comment ADD article_id integer default 1
CONSTRAINT article_id REFERENCES article NOT NULL;

CREATE TRIGGER set_comment_timestamp
BEFORE UPDATE ON comment
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();