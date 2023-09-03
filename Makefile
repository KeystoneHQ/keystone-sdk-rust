#SHELL := /bin/bash

all: android

android:
	@echo "Step: Generating Android builds"
	@echo "1: arm64-v8a"
	cargo ndk -t arm64-v8a build -p ur-registry-ffi --release
	@echo "2: armeabi-v7a"
	cargo ndk -t armeabi-v7a build -p ur-registry-ffi --release
	@echo "3: x86"
	cargo ndk -t x86 build -p ur-registry-ffi --release
	@echo "4: x86_64"
	cargo ndk -t x86_64 build -p ur-registry-ffi --release
	@echo "Android buildup"

generate_xcframework:
	@echo "Step: Generate XCFramework"
	cargo build -r --target aarch64-apple-ios --no-default-features
	cargo build -r --target x86_64-apple-ios --no-default-features
	cargo build -r --target aarch64-apple-ios-sim --no-default-features
	mkdir -p target/sim
	lipo target/aarch64-apple-ios-sim/release/libur_registry_ffi.dylib target/x86_64-apple-ios/release/libur_registry_ffi.dylib -create -output target/sim/libur_registry_ffi.dylib
	xcodebuild -create-xcframework -library target/sim/libur_registry_ffi.dylib -headers include -library target/aarch64-apple-ios/release/libur_registry_ffi.dylib -headers include -output target/URRegistryFFI.xcframework
