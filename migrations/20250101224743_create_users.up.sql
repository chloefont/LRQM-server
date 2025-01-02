CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    bib_id VARCHAR(100) NOT NULL,
    event_id INT NOT NULL REFERENCES events(id),
    total_meters INT NOT NULL DEFAULT 0,
    UNIQUE (BIB_ID, event_id)
);