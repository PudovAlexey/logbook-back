
# APP_PATH := /home/alexey/projects/logbook/logbook-back/target/release/logbook-app-back
APP_PATH := /usr/src/app/target/release/logbook-app-back

.PHONY: make-migration

run:
	@cargo run

release:
	@cargo build --release

debug:
	@cargo build --debug

watch:
	@cargo watch -x run

db-setup:
	@diesel setup

make-migration:
	@diesel migration generate $(MIGRATION_NAME)

migration-up:
	@diesel migration run

migration-down:
	@diesel migration redo

app-build:
	@cargo build

db-cli-intall:
	@cargo install diesel_cli


run-app:
	@echo "Запуск приложения..."
	@$(APP_PATH)

initialize-app: db-setup migration-up run-app
