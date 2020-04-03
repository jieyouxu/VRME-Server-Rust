CREATE TABLE IF NOT EXISTS meeting_sessions (
	meeting_id UUID primary key,
	presenter UUID NOT NULL,
	listeners UUID[] NOT NULL,
	started_at TIMESTAMP NOT NULL,
	CONSTRAINT unique_presenter UNIQUE (presenter)
);
