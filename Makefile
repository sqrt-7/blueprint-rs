test-coverage:
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
	grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html

check-lint:
	cargo clippy -- --no-deps -D warnings

check-format:
	cargo fmt -- --check

check-audit:
	cargo audit

check: check-lint check-format check-audit

build:
	cargo build --verbose

test:
	cargo test --verbose