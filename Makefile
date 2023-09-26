all:
	cargo clean && cargo build --release
	ls -l ./target/armv5te-unknown-linux-musleabi/release/armv5te_demo
	