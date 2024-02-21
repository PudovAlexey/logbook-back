
.PHONY: make-migration

run:
	@cargo run

watch:
	@cargo watch -x run

make-migration:
	@diesel migration generate $(MIGRATION_NAME)
