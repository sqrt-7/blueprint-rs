test-coverage:
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
	grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html

check-cargo:
	cargo check

check-lint:
	cargo clippy -- --no-deps

check-format:
	cargo fmt -- --check

check-audit:
	cargo audit

check: check-cargo check-lint check-format check-audit

build:
	cargo build --verbose

test:
	cargo test -- --nocapture --test-threads 1

grpcui:
	grpcui -plaintext -v -proto blueprint.proto 127.0.0.1:9000