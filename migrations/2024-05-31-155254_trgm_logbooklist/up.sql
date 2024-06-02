CREATE EXTENSION pg_trgm;

CREATE INDEX trgm_loginfo_title_idx ON loginfo USING gist (title gist_trgm_ops);
CREATE INDEX trgm_loginfo_description_idx ON loginfo USING gist (description gist_trgm_ops);