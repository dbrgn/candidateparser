all:
	@echo "Make targets:"
	@echo ""
	@echo " - android"
	@echo " - android-armv7"
	@echo " - android-aarch64"
	@echo " - android-x86"
	@echo " - ios"
	@echo " - ios-universal"
	@echo ""
	@echo "Type 'make <target>' to build."

android-armv7:
	CC=$(shell pwd)/NDK/arm/bin/arm-linux-androideabi-clang cargo build --package candidateparser-jni --release --target armv7-linux-androideabi
	NDK/arm/arm-linux-androideabi/bin/strip target/armv7-linux-androideabi/release/libcandidateparser_jni.so

android-aarch64:
	CC=$(shell pwd)/NDK/arm64/bin/aarch64-linux-android-clang cargo build --package candidateparser-jni --release --target aarch64-linux-android
	NDK/arm64/aarch64-linux-android/bin/strip target/aarch64-linux-android/release/libcandidateparser_jni.so

android-x86:
	CC=$(shell pwd)/NDK/x86/bin/i686-linux-android-clang cargo build --package candidateparser-jni --release --target i686-linux-android
	NDK/x86/i686-linux-android/bin/strip target/i686-linux-android/release/libcandidateparser_jni.so

ios-universal:
	cd candidateparser-ffi && cargo lipo --release

android: android-armv7 android-aarch64 android-x86

ios: ios-universal

clean:
	rm -r target/armv7-linux-androideabi target/aarch64-linux-android target/i686-linux-android

.PHONY: clean all android ios
