export QT_QPA_PLATFORM := "xcb"
export ANDROID_NDK_HOME := "/opt/android-ndk"

emulator:
    emulator -avd Pixel_9 -netdelay none -netspeed full

mobile:
    dx --verbose serve --hot-patch --package app --platform android --target aarch64-linux-android

web:
    dx --verbose serve --port 8080 --hot-patch --package app --platform web

desktop:
    dx serve --hot-patch  --package app --platform desktop
