CREATE TABLE IF NOT EXISTS fahrzeuge (
  model varchar(255) NOT NULL,
  id SERIAL PRIMARY KEY,
  marke varchar(255) NOT NULL,
  leistung int NOT NULL,
  active boolean DEFAULT FALSE,
  plz int NOT NULL);

INSERT INTO fahrzeuge(model, marke, leistung, active, plz) VALUES ('m3','BMW',450, TRUE, 50667),('RS6','Audi',550, TRUE, 10115),('Model S','Tesla',550, TRUE, 60306);
