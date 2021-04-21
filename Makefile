rpi4: build-openssl-rpi4 build-rpi4

build-rpi4:
	cargo build --release --target=aarch64-unknown-linux-gnu

build-openssl-rpi4:
	bash ./scripts/build-openssl-aarch64.sh
