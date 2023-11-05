CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_users_timestamp
	BEFORE UPDATE ON users
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TRIGGER set_users_timestamp
	BEFORE UPDATE ON adjustments
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TRIGGER set_users_timestamp
	BEFORE UPDATE ON sessions
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TRIGGER set_users_timestamp
	BEFORE UPDATE ON corrections
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TRIGGER set_users_timestamp
	BEFORE UPDATE ON location_trackers
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TRIGGER set_users_timestamp
	BEFORE UPDATE ON pins
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();
