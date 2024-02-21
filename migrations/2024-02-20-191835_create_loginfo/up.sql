CREATE TABLE loginfo (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    depth FLOAT(2),
    start_datetime TIMESTAMP,
    end_datetime TIMESTAMP,
    water_temperature FLOAT(2),
    vawe_power FLOAT(2),
    side_view FLOAT(2),
    start_pressure INT,
    end_pressure INT,
    description VARCHAR
)
