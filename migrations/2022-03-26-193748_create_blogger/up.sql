CREATE TABLE blogger (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    UNIQUE (username),
    UNIQUE (email)
);


INSERT INTO blogger (username, email, password)
VALUES ('placeholder', 'placeholder@gmail.com', 'placeholderpass');
