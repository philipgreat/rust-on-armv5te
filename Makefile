all:
	cargo clean && cargo build --release
	scp -P 6822 ./target/armv5te-unknown-linux-musleabi/release/armv5te_demo root@iot.doublechaintech.com:~/