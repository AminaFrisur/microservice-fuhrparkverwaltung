FROM mysql/mysql-server:latest
LABEL description="Docker File for Microservice Fuhrpark"

# Environment variables
ENV MYSQL_DATABASE fuhrpark
ENV MYSQL_ROOT_PASSWORD secretadmin ${PWD}

# Set working directory
WORKDIR ./Datenbank/mysql

# Create the Database
RUN mkdir /mysql
RUN mkdir /mysql/database
RUN chmod 644 /mysql/database

COPY ./Datenbank/sqlData/data.sql /docker-entrypoint-initdb.d/



# FROM rust:1.31
# WORKDIR ./Datenbank/src
