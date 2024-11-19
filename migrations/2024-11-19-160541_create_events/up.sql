CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL, 
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,
    meters_goal INT NOT NULL
);