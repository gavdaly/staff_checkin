CREATE VIEW timesheet AS
	SELECT u.id, u.first_name, u.last_name, u.state, date_trunc('day', s.start_time) as session_date, s.id as session_id, s.start_time, s.end_time, s.state as session_state, adj.start_date as adjustment_date, adj.id as adjustment_id, adj.category, adj.reason, adj.response, adj.state as adjustment_state, cor.reason as correction_reason, cor.response as correction_response
		FROM users as u
	INNER JOIN sessions as s
		ON u.id = s.user_id
	INNER JOIN adjustments as adj
		ON u.id = adj.user_id
	LEFT JOIN corrections as cor
		ON s.id = cor.session_id;
