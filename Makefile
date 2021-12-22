.PHONY: run
run:
	cargo run

.PHONY: watch
watch:
	cargo watch -x 'run'

.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo test --test entry_test

.PHONY: migrate
migrate:
	diesel migration run

.PHONY: migrate-redo
migrate-redo:
	diesel migration redo

.PHONY: reset-db
reset-db:
	diesel database reset