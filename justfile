export QT_QPA_PLATFORM := "xcb"
export ANDROID_NDK_HOME := "/opt/android-ndk"

emulator:
    emulator -avd Pixel_7 -netdelay none -netspeed full

mobile:
    dx serve --package web --platform android
