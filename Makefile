all: android

android-armv7:
	cargo build --package candidateparser-ffi --release --target armv7-linux-androideabi

android-aarch64:
	cargo build --package candidateparser-ffi --release --target aarch64-linux-android

android-x86:
	cargo build --package candidateparser-ffi --release --target i686-linux-android

android: android-armv7 android-aarch64 android-x86
