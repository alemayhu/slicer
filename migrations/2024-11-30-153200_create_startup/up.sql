-- Your SQL goes here

CREATE TABLE runs (
    id SERIAL PRIMARY KEY,
    start_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP,
    status VARCHAR(50) NOT NULL DEFAULT 'running',
    description TEXT,
    parameters JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create an index on status for faster queries
CREATE INDEX idx_runs_status ON runs(status);

-- Create an index on start_time for time-based queries
CREATE INDEX idx_runs_start_time ON runs(start_time);
