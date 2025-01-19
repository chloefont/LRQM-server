CREATE TABLE measures (
    id SERIAL PRIMARY KEY,
    start_time TIMESTAMP NOT NULL DEFAULT NOW(),
    end_time TIMESTAMP,
    meters INT NOT NULL DEFAULT 0,
    user_id INT NOT NULL REFERENCES users(id)
);