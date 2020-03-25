CREATE TABLE IF NOT EXISTS auth_sessions (
	user_id UUID PRIMARY KEY,
	auth_token VARCHAR(32) NOT NULL,
	last_used TIMESTAMP NOT NULL
);
