all:
	cargo clean && cargo build --release
	ls -l ./target/release/armv5te_demo
	scp ./target/release/armv5te_demo ubuntu@mq.teaql.io:/var/www/html

	
