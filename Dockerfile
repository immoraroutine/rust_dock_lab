FROM rust:1.79-bookworm

WORKDIR /api

RUN cargo install cargo-watch

COPY ./ /api
RUN cargo install --path .

EXPOSE 8080

CMD [ "cargo" "run" ]