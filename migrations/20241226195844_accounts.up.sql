-- Add up migration script here
CREATE TABLE "accounts" (
  "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  "role" varchar NOT NULL DEFAULT 'user',
  "password_hash" varchar NOT NULL,
  "email" varchar UNIQUE NOT NULL,
  "secret_key" varchar,
  "verification_sent" timestamptz,
  "email_verified_at" timestamptz,
  "created_by" varchar NOT NULL DEFAULT 'system',
  "created_at" timestamptz NOT NULL DEFAULT (now()),
  "changed_by" varchar NOT NULL DEFAULT 'system',
  "changed_at" timestamptz NOT NULL DEFAULT (now())
);

CREATE TABLE "sessions" (
  "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  "account_id" uuid NOT NULL,
  "user_agent" varchar NOT NULL,
  "client_ip" varchar NOT NULL,
  "refresh_token" varchar NOT NULL,
  "is_blocked" boolean NOT NULL DEFAULT false,
  "expires_at" timestamptz NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT (now())
);


ALTER TABLE "sessions" ADD FOREIGN KEY ("account_id") REFERENCES "accounts" ("id");

INSERT INTO "accounts" ("role", "password_hash", "email") VALUES ('admin', '$argon2id$v=19$m=19456,t=2,p=1$U6VJ7kZz08A0OLsvbAkUsg$KGGWrLiRHwSSqLxoBxdLcHBv3gv/CJrADXNIeXNWU8Q', 'admin');