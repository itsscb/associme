-- Add up migration script here
CREATE TABLE "members" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    "first_name" varchar(50) NOT NULL,
    "last_name" varchar(50) NOT NULL,
    "email" varchar(100) UNIQUE NOT NULL,
    "phone" varchar(30) NOT NULL,
    "member_id" INT CHECK (member_id > 0),
    "birthday" timestamptz NOT NULL,
    "postalcode" varchar(15) NOT NULL,
    "city" varchar(50) NOT NULL,
    "street" varchar(100) NOT NULL,
    "house_number" varchar(10) NOT NULL,
    "membership_state" varchar(20) NOT NULL DEFAULT 'none',
    "resignation_date" timestamptz,
    "resignation_reason" varchar,
    "created_by" varchar NOT NULL DEFAULT 'system',
    "created_at" timestamptz NOT NULL DEFAULT (now()),
    "changed_by" varchar NOT NULL DEFAULT 'system',
    "changed_at" timestamptz NOT NULL DEFAULT (now())
);
