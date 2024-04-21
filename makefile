
.PHONY: make-migration

run:
	@cargo run

release:
	@cargo build --release

debug:
	@cargo build --debug

watch:
	@cargo watch -x run

make-migration:
	@diesel migration generate $(MIGRATION_NAME)

migration-up:
	@diesel migration run

migration-down:
	@diesel migration redo
