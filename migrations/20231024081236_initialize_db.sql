-- BEGIN TABLE public.adjustments
DROP TABLE IF EXISTS public.adjustments CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.adjustments (
	integer_id bigint,
	category integer,
	start_date date,
	end_date date,
	duration integer,
	reason text,
	response text,
	state integer,
	user_integer_id bigint,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	user_id uuid,
	PRIMARY KEY(id)
);

COMMIT;


-- BEGIN TABLE public.api_keys
DROP TABLE IF EXISTS public.api_keys CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.api_keys (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	name character varying NOT NULL,
	secret_digest character varying NOT NULL,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- Table public.api_keys contains no data. No inserts have been genrated.
-- Inserting 0 rows into public.api_keys


-- END TABLE public.api_keys

-- BEGIN TABLE public.ar_internal_metadata
DROP TABLE IF EXISTS public.ar_internal_metadata CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.ar_internal_metadata (
	"key" character varying NOT NULL,
	"value" character varying,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	PRIMARY KEY("key")
);

COMMIT;



-- BEGIN TABLE public.assignations
DROP TABLE IF EXISTS public.assignations CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.assignations (
	integer_id bigint,
	"key" character varying,
	start_time timestamp without time zone,
	end_time timestamp without time zone,
	state integer,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	user_id uuid,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.assignations

-- BEGIN TABLE public.corrections
DROP TABLE IF EXISTS public.corrections CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.corrections (
	integer_id bigint,
	reason text,
	response text,
	start_time timestamp without time zone,
	end_time timestamp without time zone,
	original_start_time timestamp without time zone,
	original_end_time timestamp without time zone,
	assignation_integer_id bigint,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	assignation_id uuid,
	new_start_time timestamp without time zone,
	new_end_time timestamp without time zone,
	PRIMARY KEY(id)
);

COMMIT;


-- END TABLE public.corrections

-- BEGIN TABLE public.exams
DROP TABLE IF EXISTS public.exams CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.exams (
	integer_id bigint,
	patient_name character varying,
	patient_id character varying,
	form json,
	notes text,
	state integer,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.exams

-- BEGIN TABLE public.hours
DROP TABLE IF EXISTS public.hours CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.hours (
	user_id uuid,
	monday json,
	tuesday json,
	wednesday json,
	thursday json,
	friday json,
	saturday json,
	sunday json,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- Table public.hours contains no data. No inserts have been genrated.
-- Inserting 0 rows into public.hours


-- END TABLE public.hours

-- BEGIN TABLE public.links
DROP TABLE IF EXISTS public.links CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.links (
	integer_id bigint,
	url character varying,
	title character varying,
	description text,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.links

-- BEGIN TABLE public.location_trackers
DROP TABLE IF EXISTS public.location_trackers CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.location_trackers (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	latitude numeric,
	longitude numeric,
	accuracy integer,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.location_trackers

-- BEGIN TABLE public.messages
DROP TABLE IF EXISTS public.messages CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.messages (
	integer_id bigint,
	"text" text,
	user_integer_id bigint NOT NULL,
	state integer,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	user_id uuid,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.messages

-- BEGIN TABLE public.pins
DROP TABLE IF EXISTS public.pins CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.pins (
	number character varying,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	user_id uuid,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.pins

-- BEGIN TABLE public.public_contacts
DROP TABLE IF EXISTS public.public_contacts CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.public_contacts (
	request integer,
	details json,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.public_contacts

-- BEGIN TABLE public.public_surveys
DROP TABLE IF EXISTS public.public_surveys CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.public_surveys (
	"data" json,
	state integer,
	name character varying,
	uuid character varying,
	family_id character varying,
	phone_number character varying,
	email character varying,
	prefered_contact integer,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	email_delivery integer DEFAULT 0,
	"isArcived" boolean DEFAULT false,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.public_surveys

-- BEGIN TABLE public.query_factories
DROP TABLE IF EXISTS public.query_factories CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.query_factories (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	name character varying NOT NULL,
	"version" integer NOT NULL,
	initial_data json,
	office_questions json,
	patient_questions json,
	office_output text,
	complete_output text,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.query_factories

-- BEGIN TABLE public.query_stores
DROP TABLE IF EXISTS public.query_stores CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.query_stores (
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	query_factory_id bigint NOT NULL,
	state integer DEFAULT 0 NOT NULL,
	maxident_user_id integer,
	maxident_appointment_id integer,
	initial_data json,
	patient_answers json,
	office_answers json,
	office_output text,
	complete_output text,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- Table public.query_stores contains no data. No inserts have been genrated.
-- Inserting 0 rows into public.query_stores


-- END TABLE public.query_stores

-- BEGIN TABLE public.schema_migrations
DROP TABLE IF EXISTS public.schema_migrations CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.schema_migrations (
	"version" character varying NOT NULL,
	PRIMARY KEY("version")
);

COMMIT;

-- END TABLE public.schema_migrations

-- BEGIN TABLE public.settings
DROP TABLE IF EXISTS public.settings CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.settings (
	user_id uuid,
	prefered_name character varying,
	store json,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.settings

-- BEGIN TABLE public.users
DROP TABLE IF EXISTS public.users CASCADE;
BEGIN;

CREATE TABLE IF NOT EXISTS public.users (
	integer_id bigint,
	first_name character varying,
	provider character varying,
	"key" character varying,
	phone_number character varying,
	display_name character varying,
	"role" integer,
	last_name character varying,
	api_id character varying,
	state integer,
	access_pin integer,
	created_at timestamp(6) without time zone NOT NULL,
	updated_at timestamp(6) without time zone NOT NULL,
	settings text,
	hours text,
	id uuid DEFAULT gen_random_uuid() NOT NULL,
	PRIMARY KEY(id)
);

COMMIT;

-- END TABLE public.users
