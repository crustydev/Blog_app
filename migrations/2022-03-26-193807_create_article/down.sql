ALTER TABLE article DROP COLUMN blogger_id;

DROP TABLE article;

DROP TRIGGER [IF EXISTS] set_article_timestamp ON article [ CASCADE | RESTRICT ];