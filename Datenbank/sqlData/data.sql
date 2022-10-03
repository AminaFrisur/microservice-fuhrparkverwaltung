CREATE DATABASE fuhrpark;
\c fuhrpark;
CREATE TABLE IF NOT EXISTS fahrzeuge (
  model varchar(255) NOT NULL,
  id SERIAL PRIMARY KEY NOT NULL ,
  marke varchar(255) NOT NULL,
  leistung int DEFAULT NULL);

INSERT INTO fahrzeuge VALUES ('m3',1,'BMW',450),('RS6',2,'Audi',550),('Model S',3,'Tesla',550);
