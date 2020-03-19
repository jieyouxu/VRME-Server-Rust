CREATE TABLE IF NOT EXISTS accounts (
	user_id SERIAL PRIMARY KEY,
	email VARCHAR(355) UNIQUE NOT NULL,
	first_name VARCHAR(100) NOT NULL,
	last_name VARCHAR(100) NOT NULL,
	iteration_count INT NOT NULL CHECK(iteration_count >= 0),
	salt CHAR(16) NOT NULL,
	password_hash CHAR(32) NOT NULL,
	created_at DATE NOT NULL
);
