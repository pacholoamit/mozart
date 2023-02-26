.PHONY: install-cargo-watch check-protoc install run-server run-client doc dev


install-cargo-watch:
	@which cargo-watch > /dev/null || cargo install cargo-watch

check-protoc:
	@which protoc > /dev/null || echo "Please install protoc"

install:
	@cargo install --path .
	@make install-cargo-watch
	@make check-protoc

run-server:
	@cargo watch -q -c -x "test" 
	@cargo watch -q -c -x "run --bin server"

run-client:
	@cargo watch -q -c -x "test" 
	@cargo watch -q -c -x "run --bin client"

doc:
	@cargo doc
	@cargo docserver -p 4545


dev:
	@cargo watch -q -c -x "test" 
	