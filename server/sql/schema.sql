DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE testing.counter (
	id INT PRIMARY KEY NOT NULL DEFAULT(1) CHECK (id = 1),
	count INT NOT NULL
);

INSERT INTO testing.counter(count)
VALUES (0)
RETURNING count;