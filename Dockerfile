# # Используем официальный образ Rust для сборки
# FROM rust:latest as builder

# # Устанавливаем diesel_cli и зависимости для PostgreSQL (если используете другую БД, измените пакет)
# RUN apt-get update && \
#     apt-get install -y libpq-dev && \
#     cargo install diesel_cli --no-default-features --features postgres

# # Рабочая директория
# WORKDIR /usr/src/app

# # Копируем файлы проекта
# COPY . .

# # Собираем приложение
# RUN cargo build --release

# # Запускаем инициализацию (migrations и т.д.)
# RUN make initialize-app

# # Финальный образ
# FROM debian:bullseye-slim

# # Устанавливаем зависимости для runtime (для PostgreSQL)
# RUN apt-get update && \
#     apt-get install -y libpq5 && \
#     rm -rf /var/lib/apt/lists/*

# # Копируем бинарник из builder
# COPY --from=builder /usr/src/app/target/release/your_app_name /usr/local/bin/your_app_name

# # Копируем миграции (если нужно)
# COPY --from=builder /usr/src/app/migrations /migrations

# # Рабочая директория
# WORKDIR /app

# # Порт, который будет слушать приложение
# EXPOSE 8000

# # Команда для запуска приложения
# CMD ["logbook-app-back"]


FROM rust:latest

RUN apt-get update && \
    apt-get install -y libpq-dev && \
    cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

CMD ["make", "initialize-app"]