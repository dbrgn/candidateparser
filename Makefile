all: android

android-armv7:
	CC=$(shell pwd)/NDK/arm/bin/arm-linux-androideabi-clang cargo build --package candidateparser-jni --release --target armv7-linux-androideabi

android-aarch64:
	CC=$(shell pwd)/NDK/arm64/bin/aarch64-linux-android-clang cargo build --package candidateparser-jni --release --target aarch64-linux-android

android-x86:
	CC=$(shell pwd)/NDK/x86/bin/i686-linux-android-clang cargo build --package candidateparser-jni --release --target i686-linux-android

android: android-armv7 android-aarch64 android-x86

clean:
	rm -r target/armv7-linux-androideabi target/aarch64-linux-android target/i686-linux-android

.PHONY: clean all android
