default: build

build:
	cargo build
	cp target/debug/barish barish
clean:
	cargo clean
release:
	cargo build --release
	cp target/release/barish barish
dependencies:
	cargo tree
