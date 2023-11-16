-- Add migration script here
CREATE TABLE IF NOT EXISTS magic_links (
	id uuid DEFAULT gen_random_uuid() UNIQUE NOT NULL,
	user_id uuid UNIQUE NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	updated_at timestamptz NOT NULL DEFAULT NOW(),
	PRIMARY KEY(id),
    FOREIGN KEY(user_id) REFERENCES users (id)
);

CREATE INDEX magic_link_index
ON magic_links(id);