#SHELL := /bin/bash
NAME = libur_registry_ffi
SO = $(NAME).so
LIB = $(NAME).a

ARCHS_ANDROID = aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
ARCHS_IOS = aarch64-apple-ios

# This varible is from env, when building in jitpack.
# This can only be used in development environment.
# ANDROID_DEST = ~/keystone-sdk-android/library/src/main/jniLibs

all: android

build-android: $(ARCHS_ANDROID)

clean:
	rm -R $(ANDROID_DEST)
	rm $(IOS_DEST)/LIB

# Android
aarch64-linux-android:
	cargo ndk -t arm64-v8a build --release

armv7-linux-androideabi:
	cargo ndk -t armeabi-v7a build --release

i686-linux-android:
	cargo ndk -t x86 build --release

x86_64-linux-android:
	cargo ndk -t x86_64 build --release

android: build-android
	mkdir -p $(ANDROID_DEST)
	mkdir -p $(ANDROID_DEST)/x86
	mkdir -p $(ANDROID_DEST)/x86_64
	mkdir -p $(ANDROID_DEST)/arm64-v8a
	mkdir -p $(ANDROID_DEST)/armeabi-v7a

	cp ../../target/i686-linux-android/release/$(SO) ${ANDROID_DEST}/x86/$(SO)
	cp ../../target/x86_64-linux-android/release/$(SO) ${ANDROID_DEST}/x86_64/$(SO)
	cp ../../target/aarch64-linux-android/release/$(SO) ${ANDROID_DEST}/arm64-v8a/$(SO)
	cp ../../target/armv7-linux-androideabi/release/$(SO) ${ANDROID_DEST}/armeabi-v7a/$(SO)
