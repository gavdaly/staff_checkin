CREATE TABLE IF NOT EXISTS public.adjustments (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
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
	PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS public.sessions (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	user_id uuid NOT NULL,
	start_time timestamptz NOT NULL DEFAULT NOW(),
	end_time timestamptz,
	state integer NOT NULL DEFAULT 0,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS public.corrections (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
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
	PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS public.location_trackers (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	latitude double precision NOT NULL,
	longitude double precision NOT NULL,
	accuracy double precision NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS public.pins (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	user_id uuid NOT NULL,
	number integer NOT NULL DEFAULT random() * (999999 - 100000 + 1) + 100000,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS public.users (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	phone_number varchar NOT NULL,
	first_name varchar NOT NULL,
	last_name varchar NOT NULL,
	display_name varchar,
	api_id integer,
	state integer NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id)
);