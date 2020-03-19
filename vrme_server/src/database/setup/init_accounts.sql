CREATE TABLE IF NOT EXISTS accounts (
	user_id UUID PRIMARY KEY,
	email VARCHAR(355) UNIQUE NOT NULL,
	first_name VARCHAR(100) NOT NULL,
	last_name VARCHAR(100) NOT NULL,
	iteration_count INT NOT NULL CHECK(iteration_count >= 0),
	salt BYTEA NOT NULL,
	password_hash BYTEA NOT NULL,
	created_at DATE NOT NULL
);
