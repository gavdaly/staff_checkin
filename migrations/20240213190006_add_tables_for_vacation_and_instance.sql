-- Add migration script here
CREATE TABLE IF NOT EXISTS instances (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	user_id uuid UNIQUE NOT NULL,
    category smallint NOT NULL,
    reference_id uuid,
    date_time timestamptz NOT NULL DEFAULT NOW(),
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
    FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE INDEX instance_index
ON instances(id);

CREATE TABLE IF NOT EXISTS vacations (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	user_id uuid UNIQUE NOT NULL,
    category integer NOT NULL,
    start_on date NOT NULL,
    end_on date NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
    FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE INDEX vacation_index
ON vacations(id);

CREATE TABLE IF NOT EXISTS vacation_pay (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	user_id uuid NOT NULL,
    amount varchar(50) NOT NULL,
    requested_for date NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
    FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE INDEX vacation_pay_index
ON vacation_pay(id);

CREATE TABLE IF NOT EXISTS manual_entry (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	instance_id uuid UNIQUE NOT NULL,
    category integer NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
    FOREIGN KEY(instance_id) REFERENCES instances (id)
);

CREATE INDEX manual_entry_index
ON manual_entry(id);


CREATE TABLE IF NOT EXISTS business (
    id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
    name varchar(50) NOT NULL,
    logo varchar(50) NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY(id)
);

CREATE INDEX business_index ON business(id);

CREATE TABLE IF NOT EXISTS location (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	business_id uuid NOT NULL,
    name varchar(50) NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    accepted_accuracy integer NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
    FOREIGN KEY(business_id) REFERENCES business (id)
);

CREATE INDEX location_index
ON location(id);

ALTER TABLE users ADD COLUMN IF NOT EXISTS business_id uuid;
