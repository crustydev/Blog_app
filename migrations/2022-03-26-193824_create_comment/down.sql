ALTER TABLE comment DROP COLUMN blogger_id;

ALTER TABLE comment DROP COLUMN article_id;

DROP TRIGGER [IF EXISTS] set_comment_timestamp ON article [ CASCADE | RESTRICT ];

DROP TABLE comment;
