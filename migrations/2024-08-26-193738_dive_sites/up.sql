CREATE TABLE dive_site(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR,
    latitude NUMERIC(9, 6) NOT NULL,
    longitude NUMERIC(9, 6) NOT NULL,
    is_verified BOOLEAN NOT NULL,
    depth_from FLOAT(2) NOT NULL,
    depth_to FLOAT(2) NOT NULL,
    level INTEGER NOT NULL,
    image_id INTEGER NOT NULL REFERENCES image (id)
);

ALTER TABLE loginfo
ADD COLUMN site_id INTEGER NOT NULL,
ADD CONSTRAINT fk_loginfo_dive_site FOREIGN KEY (site_id) REFERENCES dive_site(id);