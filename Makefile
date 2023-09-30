all:
	cargo clean && cargo build --release
	ls -l ./target/release/armv5te_demo
	