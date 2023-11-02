SELECT category, start_date, end_date, duration, reason, response, state, id, user_id
FROM adjustments
WHERE user_id = $1 AND start_date BETWEEN $2 AND $3