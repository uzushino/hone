CREATE TABLE downloads (
  id SERIAL PRIMARY KEY,
  version VARCHAR NOT NULL
);

INSERT INTO downloads (id, version) VALUES (1, "0.1");
INSERT INTO downloads (id, version) VALUES (2, "0.2");
INSERT INTO downloads (id, version) VALUES (3, "0.3");