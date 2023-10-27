SELECT id, session_id, start_time, end_time, original_start_time, original_end_time
FROM corrections
WHERE start_time > $1
