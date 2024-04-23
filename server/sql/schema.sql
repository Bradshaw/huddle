DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.users (
	id  BIGSERIAL PRIMARY KEY,
	email       VARCHAR(200) NOT NULL,
	first_name  VARCHAR(200) NOT NULL,
	last_name   VARCHAR(200) NOT NULL,
	username    VARCHAR(50) UNIQUE NOT NULL,
	UNIQUE (username)
);

CREATE TABLE testing.counter (
	id INT PRIMARY KEY NOT NULL DEFAULT(1) CHECK (id = 1),
	count INT NOT NULL
);

INSERT INTO testing.counter(count)
VALUES (0)
RETURNING count;