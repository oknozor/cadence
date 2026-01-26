package dev.dioxus.main

import android.Manifest
import android.content.pm.PackageManager
import android.media.AudioFormat
import android.media.AudioRecord
import android.media.MediaRecorder
import androidx.core.content.ContextCompat
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import java.io.ByteArrayOutputStream

/**
 * Audio recorder for Shazam song identification.
 * Records audio at 16kHz mono PCM 16-bit format required by Shazam.
 */
class ShazamAudioRecorder(private val activity: android.app.Activity) {
    
    companion object {
        const val SAMPLE_RATE = 16000
        const val CHANNEL_CONFIG = AudioFormat.CHANNEL_IN_MONO
        const val AUDIO_FORMAT = AudioFormat.ENCODING_PCM_16BIT
        const val BUFFER_SIZE_MULTIPLIER = 8
    }

    private val mutex = Mutex()
    private var audioRecord: AudioRecord? = null
    private var byteArrayOutputStream: ByteArrayOutputStream? = null

    private val _buffer = MutableStateFlow<ByteArray>(ByteArray(0))
    val buffer: StateFlow<ByteArray> = _buffer

    private val _duration = MutableStateFlow(0)
    val duration: StateFlow<Int> = _duration

    private val _active = MutableStateFlow(false)
    val active: StateFlow<Boolean> = _active

    /**
     * Checks if RECORD_AUDIO permission is granted.
     */
    fun hasPermission(): Boolean {
        return ContextCompat.checkSelfPermission(
            activity,
            Manifest.permission.RECORD_AUDIO
        ) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * Starts recording audio.
     * @return true if recording started successfully, false otherwise.
     */
    suspend fun start(): Boolean = mutex.withLock {
        if (_active.value) return true
        if (!hasPermission()) return false

        try {
            val bufferSize = AudioRecord.getMinBufferSize(
                SAMPLE_RATE,
                CHANNEL_CONFIG,
                AUDIO_FORMAT
            ) * BUFFER_SIZE_MULTIPLIER

            audioRecord = AudioRecord(
                MediaRecorder.AudioSource.MIC,
                SAMPLE_RATE,
                CHANNEL_CONFIG,
                AUDIO_FORMAT,
                bufferSize
            )

            if (audioRecord?.state != AudioRecord.STATE_INITIALIZED) {
                audioRecord?.release()
                audioRecord = null
                return false
            }

            byteArrayOutputStream = ByteArrayOutputStream()
            audioRecord?.startRecording()
            _active.value = true
            _duration.value = 0
            _buffer.value = ByteArray(0)

            // Start reading audio data in background
            withContext(Dispatchers.IO) {
                val buffer = ByteArray(bufferSize)
                while (_active.value && audioRecord?.recordingState == AudioRecord.RECORDSTATE_RECORDING) {
                    val bytesRead = audioRecord?.read(buffer, 0, buffer.size) ?: -1
                    if (bytesRead > 0) {
                        byteArrayOutputStream?.write(buffer, 0, bytesRead)
                        _buffer.value = byteArrayOutputStream?.toByteArray() ?: ByteArray(0)
                        _duration.value = (_buffer.value.size / 2) / SAMPLE_RATE
                    }
                }
            }

            true
        } catch (e: Exception) {
            e.printStackTrace()
            stop()
            false
        }
    }

    /**
     * Stops recording and releases resources.
     */
    suspend fun stop() {
        // Set active to false FIRST to break the recording loop in start()
        // This must happen BEFORE acquiring the mutex to avoid deadlock
        _active.value = false

        mutex.withLock {
            try {
                audioRecord?.stop()
                audioRecord?.release()
            } catch (e: Exception) {
                e.printStackTrace()
            }
            audioRecord = null
            // Don't clear byteArrayOutputStream here - we need it for getAudioData()
        }
    }

    /**
     * Gets the current recorded audio buffer as ShortArray.
     * Converts from ByteArray (PCM 16-bit little-endian) to ShortArray.
     */
    fun getAudioData(): ShortArray {
        val bytes = _buffer.value
        if (bytes.isEmpty()) return ShortArray(0)
        val shorts = ShortArray(bytes.size / 2)
        for (i in shorts.indices) {
            // Little-endian: low byte first, high byte second
            shorts[i] = ((bytes[i * 2 + 1].toInt() shl 8) or (bytes[i * 2].toInt() and 0xFF)).toShort()
        }
        return shorts
    }
}

