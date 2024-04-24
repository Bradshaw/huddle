-- Your SQL goes here
CREATE TABLE "counter"(
	"id" INT4 NOT NULL PRIMARY KEY DEFAULT(1) CHECK (id = 1),
	"count" INT4 NOT NULL
);

INSERT INTO counter(count)
VALUES (0)
RETURNING count;
