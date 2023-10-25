SELECT id, user_id, state, start_time, end_time
FROM assignations
WHERE start_time > $1
