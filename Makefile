.PHONY: check-cargo-watch run


check-cargo-watch:
	@which cargo-watch > /dev/null || cargo install cargo-watch

run-server:
	@cargo watch -q -c -x "run --bin mozart-server"

run-client:
	@cargo watch -q -c -x "run --bin mozart-client"

doc:
	@cargo doc
	@cargo docserver -p 4545
	
dev:
	@make check-cargo-watch
	@cargo watch -q -c -x "test" 
	