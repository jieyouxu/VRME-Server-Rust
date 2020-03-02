CREATE TABLE accounts (
	uuid UUID PRIMARY KEY,
	first_name VARCHAR NOT NULL,
	last_name VARCHAR NOT NULL,
	email VARCHAR NOT NULL UNIQUE,
	iteration_count INTEGER NOT NULL,
	salt CHAR(16) NOT NULL,
	password_hash CHAR(32) NOT NULL
)
