.PHONY: all install install-cargo-watch check-protoc run-server run-client doc


install-cargo-watch:
	@which cargo-watch > /dev/null || cargo install cargo-watch

check-protoc:
	@which protoc > /dev/null || echo "Please install protoc"

install:
	@cargo install --path .
	@make install-cargo-watch
	@make check-protoc

run-server:
	@cargo watch -q -c -x "run --bin mozart-server"

run-client:
	@cargo watch -q -c -x "run --bin mozart-client"

test:
	@cargo watch -q -c -x "test --all"
doc:
	@cargo doc
	@cargo docserver -p 4545

	