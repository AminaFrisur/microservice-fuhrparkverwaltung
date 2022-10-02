FROM rust:latest
LABEL description="Rest Api of Microservice Fuhrpark"

# Set working directory
WORKDIR ./rest-api/

RUN mkdir /rest-api/src/

COPY ./Cargo.toml .
COPY ./src/ ./src

RUN cargo install --path /rest-api

CMD ["restapi"]


