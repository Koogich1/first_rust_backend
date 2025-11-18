-- Your SQL goes here

CREATE TABLE "users"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

