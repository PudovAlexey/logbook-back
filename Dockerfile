FROM rust:1.75.0

WORKDIR /app

COPY . .

RUN cargo build --release

RUN cargo install diesel_cli --no-default-features --features "postgres"

WORKDIR /app/target/release

CMD ["diesel", "migration", "run"]

ENTRYPOINT ["./logbook-app-back"]