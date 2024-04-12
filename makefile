default: build

build:
	cargo build
clean:
	cargo clean
release:
	cargo build --release
dependencies:
	cargo tree
