CREATE TABLE IF NOT EXISTS users (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	phone_number varchar(10) UNIQUE NOT NULL,
	first_name varchar(50) NOT NULL,
	last_name varchar(50) NOT NULL,
	display_name varchar(15),
	api_id integer,
	state integer NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id)
);

CREATE INDEX user_index
ON users(id, phone_number, state);

CREATE TABLE IF NOT EXISTS adjustments (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	user_id uuid NOT NULL,
	category integer NOT NULL,
	start_date date NOT NULL,
	end_date date,
	duration integer NOT NULL DEFAULT 0,
	reason text NOT NULL DEFAULT '',
	response text NOT NULL DEFAULT '',
	state integer NOT NULL DEFAULT 0,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
	FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE INDEX adjustments_index
ON adjustments(user_id, start_date, state);

CREATE TABLE IF NOT EXISTS sessions (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	user_id uuid NOT NULL,
	start_time timestamptz NOT NULL DEFAULT NOW(),
	end_time timestamptz,
	state integer NOT NULL DEFAULT 0,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
	FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE INDEX sessions_index
ON sessions(user_id, start_time, state);

CREATE TABLE IF NOT EXISTS corrections (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	session_id uuid NOT NULL,
	start_time timestamptz NOT NULL,
	end_time timestamptz NOT NULL,
	original_start_time timestamptz NOT NULL,
	original_end_time timestamptz NOT NULL,
	new_start_time timestamptz NOT NULL,
	new_end_time timestamptz NOT NULL,
	reason text NOT NULL DEFAULT '',
	response text NOT NULL DEFAULT '',
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY (id),
	FOREIGN KEY (session_id) REFERENCES sessions (id)
);

CREATE INDEX corrections_index
ON corrections(session_id);

CREATE TABLE IF NOT EXISTS location_trackers (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	latitude double precision NOT NULL,
	longitude double precision NOT NULL,
	accuracy double precision NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS pins (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	user_id uuid NOT NULL,
	number integer NOT NULL DEFAULT random() * (999999 - 100000 + 1) + 100000,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
	FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX pins_index
ON pins(user_id);
