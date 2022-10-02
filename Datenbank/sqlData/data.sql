CREATE USER 'root'@'%' IDENTIFIED BY 'test';
GRANT ALL PRIVILEGES ON *.* TO 'root'@'%' WITH GRANT OPTION;
CREATE DATABASE IF NOT EXISTS fuhrpark;
USE fuhrpark;
DROP TABLE IF EXISTS `fahrzeuge`;
CREATE TABLE `fahrzeuge` (
  `model` varchar(255) NOT NULL,
  `id` int NOT NULL AUTO_INCREMENT,
  `marke` varchar(255) NOT NULL,
  `leistung` int DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

LOCK TABLES `fahrzeuge` WRITE;
INSERT INTO `fahrzeuge` VALUES ('m3',1,'BMW',450),('RS6',2,'Audi',550),('Model S',3,'Tesla',550);
UNLOCK TABLES;
