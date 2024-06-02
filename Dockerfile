# FROM rust:1.75.0

# WORKDIR /app

# COPY . .

# RUN cargo build --release

# RUN cargo install diesel_cli --no-default-features --features "postgres"

# WORKDIR /app/target/release

# CMD ["diesel", "migration", "run"]

# ENTRYPOINT ["./logbook-app-back"]



# FROM rust:1.75.0 as builder

# WORKDIR /usr/app

# COPY ./Cargo.toml .

# RUN mkdir src

# RUN echo "fn main() {}" > src/main.rs       

# RUN cargo build --release



# RUN rm src/main.rs     
# # RUN rm -R src

# COPY ./src ./src

# COPY ./migrations ./migrations

# RUN cargo build --release

# FROM rust:1.75.0

# RUN cargo install diesel_cli --no-default-features --features "postgres"

# COPY --from=builder /usr/app/target/release/logbook-app-back /usr/local/bin/logbook-app-back
# COPY --from=builder /usr/app/migrations /usr/local/bin/migrations

# WORKDIR /usr/local/bin

# # ENTRYPOINT ["diesel", "migration", "run"]

# # CMD ["chmod", "+x", "logbook-app-back", "&&", "./logbook-app-back"]
# ENTRYPOINT [ "./logbook-app-back" ]

FROM rust:1.78.0 as builder

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM rust:1.78.0

# Установка diesel_cli для работы с миграциями
RUN cargo install diesel_cli --no-default-features --features "postgres"

# Копирование исполняемого файла и миграций из билдера
COPY --from=builder /usr/src/app/target/release/logbook-app-back /usr/local/bin/logbook-app-back
COPY --from=builder /usr/src/app/migrations /migrations

WORKDIR /usr/local/bin

# Создание скрипта для запуска миграций и приложения
RUN echo '#!/bin/sh\n\
diesel migration run && \\\n\
./logbook-app-back' > entrypoint.sh

# Делаем скрипт исполняемым
RUN chmod +x entrypoint.sh

# Запуск скрипта при старте контейнера
ENTRYPOINT ["./entrypoint.sh"]
