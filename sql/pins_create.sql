-- update to automatically create a pin  random_between(100000,999999)
-- use phone number instead of user_id
BEGIN;
DELETE FROM pins WHERE user_id = $1;
    INSERT
        INTO pins
            (user_id, number)
        VALUES
            ($1, $2)
RETURNING *;

COMMIT;
