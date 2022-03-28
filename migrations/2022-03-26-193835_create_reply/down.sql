ALTER TABLE reply DROP COLUMN blogger_id;

ALTER TABLE reply DROP COLUMN comment_id;

DROP TRIGGER [IF EXISTS] set_reply_timestamp ON article [ CASCADE | RESTRICT ];

DROP TABLE reply;
