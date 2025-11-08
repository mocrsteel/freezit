-- Your SQL goes here

CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"name" VARCHAR(150) NOT NULL,
	"email" VARCHAR(250) NOT NULL
);

