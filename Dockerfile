FROM rust:1.78-bookworm

WORKDIR /api

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features "postgres"

COPY ./ /api
RUN cargo install --path .

EXPOSE 8080

CMD [ "cargo" "run" ]