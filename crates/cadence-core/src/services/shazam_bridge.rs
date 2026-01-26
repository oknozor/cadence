//! JNI bridge for Shazam song identification on Android.
//!
//! This module handles communication between Rust and Kotlin for audio recording.
//! Audio fingerprinting is now done entirely in Rust using the shazam-fingerprint crate.

use flume::Sender;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JShortArray, JString, JValue};
use jni::sys::jint;
use once_cell::sync::OnceCell;

/// Result from Shazam signature generation
#[derive(Debug, Clone)]
pub enum ShazamBridgeMessage {
    /// Signature generated successfully
    SignatureReady {
        /// Base64-encoded signature
        signature: String,
        /// Duration of audio sample in milliseconds
        sample_ms: i32,
    },
    /// Error during recording or signature generation
    Error(String),
    /// Recording started
    RecordingStarted,
    /// Recording stopped
    RecordingStopped,
}

/// Global sender for Shazam messages from Kotlin to Rust
pub static SHAZAM_MSG_TX: OnceCell<Sender<ShazamBridgeMessage>> = OnceCell::new();

/// Initialize the Shazam bridge with a message sender
pub fn init(sender: Sender<ShazamBridgeMessage>) {
    SHAZAM_MSG_TX.get_or_init(|| sender);
}

/// Send a Shazam message from JNI callback to Rust
fn send_shazam_message(msg: ShazamBridgeMessage) {
    if let Some(tx) = SHAZAM_MSG_TX.get() {
        if let Err(e) = tx.send(msg) {
            log::error!("Failed to send Shazam message: {}", e);
        }
    } else {
        log::warn!("Shazam bridge not initialized");
    }
}

/// Called from Kotlin when audio signature is generated successfully
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_dioxus_main_ShazamBridgeKt_nativeOnSignatureReady(
    mut env: JNIEnv,
    _class: JClass,
    signature: JString,
    sample_ms: jint,
) {
    let signature: String = env
        .get_string(&signature)
        .map(|s| s.into())
        .unwrap_or_default();

    log::info!("Rust received Shazam signature ({} ms)", sample_ms);
    send_shazam_message(ShazamBridgeMessage::SignatureReady {
        signature,
        sample_ms,
    });
}

/// Called from Kotlin when an error occurs during recording/signature generation
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_dioxus_main_ShazamBridgeKt_nativeOnError(
    mut env: JNIEnv,
    _class: JClass,
    error: JString,
) {
    let error: String = env
        .get_string(&error)
        .map(|s| s.into())
        .unwrap_or_else(|_| "Unknown error".to_string());

    log::error!("Rust received Shazam error: {}", error);
    send_shazam_message(ShazamBridgeMessage::Error(error));
}

/// Called from Kotlin when recording starts
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_dioxus_main_ShazamBridgeKt_nativeOnRecordingStarted(
    _env: JNIEnv,
    _class: JClass,
) {
    log::info!("Rust received: Shazam recording started");
    send_shazam_message(ShazamBridgeMessage::RecordingStarted);
}

/// Called from Kotlin when recording stops
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_dioxus_main_ShazamBridgeKt_nativeOnRecordingStopped(
    _env: JNIEnv,
    _class: JClass,
) {
    log::info!("Rust received: Shazam recording stopped");
    send_shazam_message(ShazamBridgeMessage::RecordingStopped);
}

/// Called from Kotlin when raw audio data is ready for fingerprinting.
/// This is the new approach - fingerprinting is done entirely in Rust.
///
/// # Arguments
/// * `audio_data` - Raw PCM audio samples (16-bit signed, mono, 16kHz)
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_dioxus_main_ShazamBridgeKt_nativeOnAudioReady(
    mut env: JNIEnv,
    _class: JClass,
    audio_data: JShortArray,
) {
    // Get the length of the audio data
    let len = match env.get_array_length(&audio_data) {
        Ok(len) => len as usize,
        Err(e) => {
            log::error!("Failed to get audio array length: {}", e);
            send_shazam_message(ShazamBridgeMessage::Error(
                "Failed to get audio array length".to_string(),
            ));
            return;
        }
    };

    if len == 0 {
        log::error!("Empty audio data received");
        send_shazam_message(ShazamBridgeMessage::Error(
            "No audio data recorded".to_string(),
        ));
        return;
    }

    // Copy the audio data from Java array to Rust Vec
    let mut audio_buffer = vec![0i16; len];
    if let Err(e) = env.get_short_array_region(&audio_data, 0, &mut audio_buffer) {
        log::error!("Failed to copy audio data: {}", e);
        send_shazam_message(ShazamBridgeMessage::Error(
            "Failed to copy audio data".to_string(),
        ));
        return;
    }

    log::info!(
        "Rust received {} audio samples ({:.2}s at 16kHz)",
        len,
        len as f32 / 16000.0
    );

    // Debug: Check if audio data is silent or has actual content
    // Use i32 to avoid overflow when calling abs() on i16::MIN
    let max_amplitude = audio_buffer.iter().map(|s| (*s as i32).abs()).max().unwrap_or(0);
    let avg_amplitude: i32 =
        audio_buffer.iter().map(|s| (*s as i32).abs()).sum::<i32>() / len as i32;
    log::info!(
        "Audio stats: max_amplitude={}, avg_amplitude={} (silent if near 0)",
        max_amplitude,
        avg_amplitude
    );

    // Generate fingerprint using native Rust implementation
    let signature_uri = shazam_fingerprint::generate_signature(&audio_buffer);

    // Extract just the base64 part (remove the data URI prefix)
    let signature = signature_uri
        .strip_prefix("data:audio/vnd.shazam.sig;base64,")
        .unwrap_or(&signature_uri)
        .to_string();

    // Calculate sample duration in milliseconds
    let sample_ms = (len as i32 * 1000) / 16000;

    log::info!(
        "Generated Shazam signature in Rust ({} ms, {} bytes)",
        sample_ms,
        signature.len()
    );

    send_shazam_message(ShazamBridgeMessage::SignatureReady {
        signature,
        sample_ms,
    });
}

/// Start Shazam identification by calling Kotlin
///
/// This triggers the Kotlin side to:
/// 1. Start audio recording (using Android AudioRecord API)
/// 2. Record for the specified duration
/// 3. Send raw PCM audio data back to Rust via `nativeOnAudioReady`
/// 4. Rust generates the fingerprint and calls the Shazam API
pub fn start_identification(duration_seconds: i32) -> jni::errors::Result<()> {
    log::info!(
        "[Shazam] start_identification called with duration={}",
        duration_seconds
    );

    let ctx = ndk_context::android_context();
    log::info!("[Shazam] Got android context");

    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    log::info!("[Shazam] Got JavaVM");

    let mut env = vm.attach_current_thread().unwrap();
    log::info!("[Shazam] Attached to JNI thread");

    let context = unsafe { JObject::from_raw(ctx.context().cast()) };

    // Load ShazamBridge class via classloader
    log::info!("[Shazam] Getting class loader...");
    let class_loader = env
        .call_method(&context, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?
        .l()?;
    log::info!("[Shazam] Got class loader");

    let class_name = env.new_string("dev.dioxus.main.ShazamBridgeKt")?;
    log::info!("[Shazam] Loading ShazamBridgeKt class...");
    let bridge_class = env
        .call_method(
            &class_loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[JValue::Object(&class_name)],
        )?
        .l()?;
    log::info!("[Shazam] Class loaded successfully");

    let bridge_class = jni::objects::JClass::from(bridge_class);

    // Call startIdentification static method
    log::info!("[Shazam] Calling startIdentification on Kotlin side...");
    env.call_static_method(
        bridge_class,
        "startIdentification",
        "(Landroid/content/Context;I)V",
        &[JValue::Object(&context), JValue::Int(duration_seconds)],
    )?;

    log::info!(
        "[Shazam] Started Shazam identification for {} seconds",
        duration_seconds
    );
    Ok(())
}
