export QT_QPA_PLATFORM := "xcb"
export ANDROID_NDK_HOME := "/opt/android-ndk"

emulator:
    emulator -avd Pixel_9 -netdelay none -netspeed full

mobile:
    dx serve --package app --platform android

web:
    dx serve --package app --platform web

desktop:
    dx serve --package app --platform desktop
