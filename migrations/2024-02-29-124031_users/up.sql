-- Your SQL goes here
CREATE TABLE "users"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"last_name" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"password" VARCHAR NOT NULL,
	"age" INT4,
	"address" VARCHAR,
	"created_at" TIMESTAMP NOT NULL,
	"modified_at" TIMESTAMP NOT NULL
);

