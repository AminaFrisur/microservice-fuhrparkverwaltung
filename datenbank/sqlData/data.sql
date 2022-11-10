CREATE USER 'root'@'%' IDENTIFIED BY 'test';
GRANT ALL PRIVILEGES ON *.* TO 'root'@'%' WITH GRANT OPTION;
CREATE DATABASE IF NOT EXISTS fuhrpark;
USE fuhrpark;
DROP TABLE IF EXISTS `fahrzeuge`;
CREATE TABLE `fahrzeuge` (
    `model` VARCHAR(255) NOT NULL,
    `id` int NOT NULL AUTO_INCREMENT,
    `marke` VARCHAR(255) NOT NULL,
    `leistung` int DEFAULT 0,
    `latitude` int DEFAULT 0,
    `longitude`int DEFAULT 0,
    `active` boolean Default TRUE,
    PRIMARY KEY (`id`))
ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
LOCK TABLES `fahrzeuge` WRITE;
INSERT INTO `fahrzeuge`(model, id, marke, leistung) VALUES ('m3',1,'BMW',450),('RS6',2,'Audi',550),('Model S',3,'Tesla',550);
UNLOCK TABLES;
GRANT ALL PRIVILEGES ON fuhrpark TO 'root'@'%' WITH GRANT OPTION;