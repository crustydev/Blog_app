CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = TO_CHAR(NOW(), 'HH24:MI:SS');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TABLE reply (
    id SERIAL PRIMARY KEY,
    content VARCHAR NOT NULL,
    created_at VARCHAR NOT NULL DEFAULT TO_CHAR(NOW(), 'HH24:MI:SS'),
    updated_at VARCHAR NOT NULL DEFAULT TO_CHAR(NOW(), 'HH24:MI:SS')
);


ALTER TABLE reply ADD blogger_id integer default 1
CONSTRAINT blogger_id REFERENCES blogger NOT NULL;

ALTER TABLE reply ADD comment_id integer default 1 
CONSTRAINT comment_id REFERENCES comment NOT NULL;



CREATE TRIGGER set_reply_timestamp
BEFORE UPDATE ON reply
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();