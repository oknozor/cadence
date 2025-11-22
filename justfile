export QT_QPA_PLATFORM := "xcb"
export ANDROID_NDK_HOME := "/opt/android-ndk"

emulator:
    emulator -avd Pixel_9 -netdelay none -netspeed full

mobile:
    dx --verbose serve --hot-patch --package app --platform android

web:
    dx --verbose serve --hot-patch --package app --platform web

desktop:
    dx serve --hot-patch --package app --platform desktop

css:
    sass -w crates/app/assets/styles/main.scss crates/app/assets/styles/main.css
    && sass -w crates/cadence-ui/assets/ui.scss crates/cadence-ui/assets/ui.css
