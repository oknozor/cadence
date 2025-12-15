export QT_QPA_PLATFORM := "xcb"
export ANDROID_NDK_HOME := "/opt/android-ndk"

emulator:
    emulator -avd Pixel_9 -netdelay none -netspeed full

logcat:
    adb logcat --pid=$(adb shell pidof org.hoohoot) -v color

mobile:
    dx --verbose serve --hot-patch --package app --platform android --target aarch64-linux-android

build-android:
    dx build --package app --platform android --features mobile --target aarch64-linux-android
    echo "Apk built in target/dx/app/debug/android/app/app/build/outputs/apk/debug/app-debug.apk"

bundle-android:
    dx bundle --package app --platform android

web:
    dx --verbose serve --port 3000 --hot-patch --package app --platform web

desktop:
    dx serve --hot-patch  --package app --platform desktop
