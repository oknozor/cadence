package dev.dioxus.main

import android.app.Activity
import android.content.Context
import android.util.Log
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

/**
 * Bridge between Rust and Kotlin for Shazam song identification.
 *
 * This orchestrates the audio recording process. Audio fingerprinting
 * is now done entirely in Rust using the shazam-fingerprint crate.
 */

private const val TAG = "ShazamBridge"
private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Main)
private var recorder: ShazamAudioRecorder? = null

/**
 * Starts the Shazam identification process.
 * Called from Rust via JNI.
 *
 * @param context Android context (Activity)
 * @param durationSeconds How long to record audio
 */
fun startIdentification(context: Context, durationSeconds: Int) {
    Log.d(TAG, "startIdentification called with duration=$durationSeconds seconds")
    scope.launch {
        try {
            val activity = context as? Activity
            if (activity == null) {
                Log.e(TAG, "Context is not an Activity")
                nativeOnError("Context is not an Activity")
                return@launch
            }
            Log.d(TAG, "Activity obtained: ${activity.javaClass.simpleName}")

            // Create recorder if needed
            if (recorder == null) {
                Log.d(TAG, "Creating new ShazamAudioRecorder")
                recorder = ShazamAudioRecorder(activity)
            }

            val rec = recorder!!

            // Check permission
            if (!rec.hasPermission()) {
                Log.e(TAG, "RECORD_AUDIO permission not granted")
                nativeOnError("RECORD_AUDIO permission not granted")
                return@launch
            }
            Log.d(TAG, "RECORD_AUDIO permission granted")

            // Start recording in a separate coroutine (it blocks while recording)
            Log.d(TAG, "Starting recording coroutine")
            val recordJob = launch(Dispatchers.IO) {
                Log.d(TAG, "Recording coroutine started, calling rec.start()")
                val started = rec.start()
                Log.d(TAG, "rec.start() returned: $started")
            }

            // Notify Rust that recording started
            Log.d(TAG, "Calling nativeOnRecordingStarted()")
            nativeOnRecordingStarted()

            // Wait for the specified duration
            Log.d(TAG, "Waiting for $durationSeconds seconds...")
            delay(durationSeconds * 1000L)
            Log.d(TAG, "Wait complete")

            // Stop recording
            Log.d(TAG, "Stopping recording")
            rec.stop()
            recordJob.cancel()
            Log.d(TAG, "Calling nativeOnRecordingStopped()")
            nativeOnRecordingStopped()

            // Get audio data
            Log.d(TAG, "Getting audio data")
            val audioData = rec.getAudioData()
            Log.d(TAG, "Audio data size: ${audioData.size} samples (${audioData.size / 16000.0}s)")

            if (audioData.isEmpty()) {
                Log.e(TAG, "No audio data recorded!")
                nativeOnError("No audio data recorded")
                return@launch
            }

            // Send raw audio data to Rust for fingerprinting
            Log.d(TAG, "Calling nativeOnAudioReady with ${audioData.size} samples")
            nativeOnAudioReady(audioData)
            Log.d(TAG, "nativeOnAudioReady returned")

        } catch (e: Exception) {
            Log.e(TAG, "Exception during identification", e)
            e.printStackTrace()
            nativeOnError(e.message ?: "Unknown error during identification")
        }
    }
    Log.d(TAG, "startIdentification coroutine launched")
}

/**
 * Stops any ongoing identification.
 * Called from Rust via JNI.
 */
fun stopIdentification() {
    scope.launch {
        try {
            recorder?.stop()
            nativeOnRecordingStopped()
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}

// Native function declarations - implemented in Rust
private external fun nativeOnAudioReady(audioData: ShortArray)
private external fun nativeOnError(error: String)
private external fun nativeOnRecordingStarted()
private external fun nativeOnRecordingStopped()

// Legacy callback - kept for backward compatibility, can be removed later
@Suppress("unused")
private external fun nativeOnSignatureReady(signature: String, sampleMs: Int)

// Load the native library at module initialization
private val nativeLibLoaded: Boolean = run {
    System.loadLibrary("dioxusmain")
    true
}

