CREATE TABLE IF NOT EXISTS fahrzeuge (
  model varchar(255) NOT NULL,
  id SERIAL PRIMARY KEY,
  marke varchar(255) NOT NULL,
  leistung int DEFAULT NULL);

INSERT INTO fahrzeuge(model, marke, leistung) VALUES ('m3','BMW',450),('RS6','Audi',550),('Model S','Tesla',550);
