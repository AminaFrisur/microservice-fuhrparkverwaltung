#!/bin/bash
docker compose stop -t 1 rest-api-fuhrparkverwaltung1
docker compose rm rest-api-fuhrparkverwaltung1
docker compose build rest-api-fuhrparkverwaltung1
docker compose up --no-start rest-api-fuhrparkverwaltung1
docker compose start rest-api-fuhrparkverwaltung1
