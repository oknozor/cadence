//! Shazam-compatible audio fingerprinting library.
//!
//! This library generates audio fingerprints compatible with the Shazam API
//! from raw PCM audio data. It is based on the SongRec project's implementation.
//!
//! # Usage
//!
//! ```rust,ignore
//! use shazam_fingerprint::generate_signature;
//!
//! // Audio must be 16kHz mono 16-bit PCM
//! let audio_samples: &[i16] = &[/* ... */];
//! let signature_uri = generate_signature(audio_samples);
//! ```

mod algorithm;
mod hanning;
mod signature;

pub use algorithm::SignatureGenerator;
pub use signature::{DecodedSignature, FrequencyBand, FrequencyPeak};

/// Generate a Shazam-compatible signature URI from raw PCM audio data.
///
/// # Arguments
/// * `s16_mono_16khz_buffer` - Raw PCM audio samples (16-bit signed, mono, 16kHz sample rate)
///
/// # Returns
/// A data URI string containing the base64-encoded signature, ready for the Shazam API.
/// Format: `data:audio/vnd.shazam.sig;base64,<base64_data>`
///
/// # Panics
/// May panic if signature encoding fails (should not happen with valid input).
pub fn generate_signature(s16_mono_16khz_buffer: &[i16]) -> String {
    let signature = SignatureGenerator::make_signature_from_buffer(s16_mono_16khz_buffer);
    signature
        .encode_to_uri()
        .expect("Failed to encode signature to URI")
}

/// Generate a Shazam-compatible signature from raw PCM audio data.
///
/// This returns the decoded signature structure, which can be useful if you need
/// access to the raw signature data or want to encode it differently.
///
/// # Arguments
/// * `s16_mono_16khz_buffer` - Raw PCM audio samples (16-bit signed, mono, 16kHz sample rate)
///
/// # Returns
/// A `DecodedSignature` containing the fingerprint data.
pub fn generate_signature_raw(s16_mono_16khz_buffer: &[i16]) -> DecodedSignature {
    SignatureGenerator::make_signature_from_buffer(s16_mono_16khz_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer() {
        let buffer: &[i16] = &[];
        let signature = generate_signature_raw(buffer);
        assert_eq!(signature.sample_rate_hz, 16000);
        assert_eq!(signature.number_samples, 0);
    }

    #[test]
    fn test_signature_generation() {
        // Generate some test audio (silence)
        let buffer: Vec<i16> = vec![0; 16000 * 5]; // 5 seconds of silence
        let uri = generate_signature(&buffer);
        assert!(uri.starts_with("data:audio/vnd.shazam.sig;base64,"));
    }

    #[test]
    fn test_signature_with_tone() {
        // Generate a simple sine wave at 440Hz
        let sample_rate = 16000.0;
        let frequency = 440.0;
        let duration_samples = 16000 * 3; // 3 seconds

        let buffer: Vec<i16> = (0..duration_samples)
            .map(|i| {
                let t = i as f32 / sample_rate;
                (f32::sin(2.0 * std::f32::consts::PI * frequency * t) * 16000.0) as i16
            })
            .collect();

        let signature = generate_signature_raw(&buffer);
        assert_eq!(signature.sample_rate_hz, 16000);
        assert_eq!(signature.number_samples, duration_samples as u32);

        // Should have detected some peaks
        let total_peaks: usize = signature
            .frequency_band_to_sound_peaks
            .values()
            .map(|v| v.len())
            .sum();
        assert!(total_peaks > 0, "Should detect peaks in a tone");
    }
}

