all:
	cargo clean && cargo build --release
	ls -l ./target/armv5te-unknown-linux-musleabi/release/armv5te_demo
	scp -P 6822 ./target/armv5te-unknown-linux-musleabi/release/armv5te_demo root@iot.doublechaintech.com:~/