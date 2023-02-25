.PHONY: check-cargo-watch run


check-cargo-watch:
	@which cargo-watch > /dev/null || cargo install cargo-watch

run-cargo-watch:
	@cargo watch -q -c -x "test" -x "run --bin mozart-server"

doc:
	@cargo doc
	@cargo docserver -p 4545
	
dev:
	@make check-cargo-watch
	@make run-cargo-watch