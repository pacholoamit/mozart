.PHONY: check-cargo-watch run


check-cargo-watch:
	@which cargo-watch > /dev/null || cargo install cargo-watch

run-cargo-watch:
	@cargo watch -q -c -x "test" -x "run"
	
dev:
	@make check-cargo-watch
	@make run-cargo-watch