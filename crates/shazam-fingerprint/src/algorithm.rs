//! Shazam fingerprint generation algorithm.
//!
//! This module implements the core fingerprinting algorithm that processes
//! raw PCM audio data (16-bit signed, mono, 16kHz) and generates a signature
//! that can be used with the Shazam recognition API.

use chfft::RFft1D;
use std::collections::HashMap;

use crate::hanning::HANNING_WINDOW_2048_MULTIPLIERS;
use crate::signature::{DecodedSignature, FrequencyBand, FrequencyPeak};

/// Generator for Shazam audio fingerprints.
///
/// This struct holds all the internal state needed to process audio samples
/// through FFT and peak detection to generate a signature.
pub struct SignatureGenerator {
    /// Ring buffer for incoming audio samples.
    ring_buffer_of_samples: Vec<i16>,
    /// Current index into the ring buffer.
    ring_buffer_of_samples_index: usize,

    /// Reordered, temporary version of the ring buffer with floats
    /// (precision needed after applying Hanning window).
    reordered_ring_buffer_of_samples: Vec<f32>,

    /// Ring buffer of FFT outputs. Lists of 1025 floats, premultiplied with
    /// a Hanning function before being passed through FFT, computed from
    /// the ring buffer every new 128 samples.
    fft_outputs: Vec<Vec<f32>>,
    /// Current index into the FFT outputs ring buffer.
    fft_outputs_index: usize,

    /// FFT object for computing real FFT of 2048 samples.
    fft_object: RFft1D<f32>,

    /// Ring buffer of spread FFT outputs (after peak spreading).
    spread_fft_outputs: Vec<Vec<f32>>,
    /// Current index into the spread FFT outputs ring buffer.
    spread_fft_outputs_index: usize,

    /// Number of spread FFTs computed so far.
    num_spread_ffts_done: u32,

    /// The signature being built.
    signature: DecodedSignature,
}

impl SignatureGenerator {
    /// Create a Shazam signature from a raw PCM audio buffer.
    ///
    /// # Arguments
    /// * `s16_mono_16khz_buffer` - Raw PCM audio data as signed 16-bit samples,
    ///   mono channel, sampled at 16kHz.
    ///
    /// # Returns
    /// A `DecodedSignature` that can be encoded to base64 for the Shazam API.
    pub fn make_signature_from_buffer(s16_mono_16khz_buffer: &[i16]) -> DecodedSignature {
        let mut this = SignatureGenerator {
            ring_buffer_of_samples: vec![0i16; 2048],
            ring_buffer_of_samples_index: 0,
            reordered_ring_buffer_of_samples: vec![0.0f32; 2048],
            fft_outputs: vec![vec![0.0f32; 1025]; 256],
            fft_outputs_index: 0,
            fft_object: RFft1D::<f32>::new(2048),
            spread_fft_outputs: vec![vec![0.0f32; 1025]; 256],
            spread_fft_outputs_index: 0,
            num_spread_ffts_done: 0,
            signature: DecodedSignature {
                sample_rate_hz: 16000,
                number_samples: s16_mono_16khz_buffer.len() as u32,
                frequency_band_to_sound_peaks: HashMap::new(),
            },
        };

        for chunk in s16_mono_16khz_buffer.chunks_exact(128) {
            this.do_fft(chunk);
            this.do_peak_spreading();
            this.num_spread_ffts_done += 1;

            if this.num_spread_ffts_done >= 46 {
                this.do_peak_recognition();
            }
        }

        this.signature
    }

    /// Perform FFT on a chunk of 128 samples.
    fn do_fft(&mut self, s16_mono_16khz_buffer: &[i16]) {
        // Copy the 128 input s16le samples to the local ring buffer
        self.ring_buffer_of_samples
            [self.ring_buffer_of_samples_index..self.ring_buffer_of_samples_index + 128]
            .copy_from_slice(s16_mono_16khz_buffer);

        self.ring_buffer_of_samples_index += 128;
        self.ring_buffer_of_samples_index &= 2047;

        // Reorder the items (put the latest data at end) and apply Hanning window
        for (index, multiplier) in HANNING_WINDOW_2048_MULTIPLIERS.iter().enumerate() {
            self.reordered_ring_buffer_of_samples[index] = self.ring_buffer_of_samples
                [(index + self.ring_buffer_of_samples_index) & 2047]
                as f32
                * multiplier;
        }

        // Perform Fast Fourier transform
        let complex_fft_results = self.fft_object.forward(&self.reordered_ring_buffer_of_samples);
        assert_eq!(complex_fft_results.len(), 1025);

        // Turn complex into reals, and put the results into a local array
        let real_fft_results = &mut self.fft_outputs[self.fft_outputs_index];

        for index in 0..=1024 {
            real_fft_results[index] = ((complex_fft_results[index].re.powi(2)
                + complex_fft_results[index].im.powi(2))
                / ((1 << 17) as f32))
                .max(0.0000000001);
        }

        self.fft_outputs_index += 1;
        self.fft_outputs_index &= 255;
    }

    /// Spread peaks in both frequency and time domains.
    fn do_peak_spreading(&mut self) {
        let real_fft_results =
            &self.fft_outputs[((self.fft_outputs_index as i32 - 1) & 255) as usize];
        let spread_fft_results = &mut self.spread_fft_outputs[self.spread_fft_outputs_index];

        // Perform frequency-domain spreading of peak values
        spread_fft_results.copy_from_slice(real_fft_results);

        for position in 0..=1022 {
            spread_fft_results[position] = spread_fft_results[position]
                .max(spread_fft_results[position + 1])
                .max(spread_fft_results[position + 2]);
        }

        // Perform time-domain spreading of peak values
        let spread_fft_results_copy = spread_fft_results.clone();

        for position in 0..=1024 {
            for former_fft_number in &[1, 3, 6] {
                let former_fft_output = &mut self.spread_fft_outputs
                    [((self.spread_fft_outputs_index as i32 - *former_fft_number) & 255) as usize];
                former_fft_output[position] =
                    former_fft_output[position].max(spread_fft_results_copy[position]);
            }
        }

        self.spread_fft_outputs_index += 1;
        self.spread_fft_outputs_index &= 255;
    }

    /// Recognize peaks in the FFT output and add them to the signature.
    fn do_peak_recognition(&mut self) {
        // Note: when subtracting an array index, casting to signed is needed
        // to avoid underflow panics at runtime.
        let fft_minus_46_idx = ((self.fft_outputs_index as i32 - 46) & 255) as usize;
        let fft_minus_49_idx = ((self.spread_fft_outputs_index as i32 - 49) & 255) as usize;

        // Collect peaks to store (to avoid borrow issues)
        let mut peaks_to_store: Vec<(usize, f32, f32, f32)> = Vec::new();

        {
            let fft_minus_46 = &self.fft_outputs[fft_minus_46_idx];
            let fft_minus_49 = &self.spread_fft_outputs[fft_minus_49_idx];

            for bin_position in 10..=1014 {
                // Ensure that the bin is large enough to be a peak
                if fft_minus_46[bin_position] >= 1.0 / 64.0
                    && fft_minus_46[bin_position] >= fft_minus_49[bin_position - 1]
                {
                    // Ensure that it is frequency-domain local minimum
                    let mut max_neighbor_in_fft_minus_49: f32 = 0.0;

                    for neighbor_offset in &[-10, -7, -4, -3, 1, 2, 5, 8] {
                        max_neighbor_in_fft_minus_49 = max_neighbor_in_fft_minus_49
                            .max(fft_minus_49[(bin_position as i32 + *neighbor_offset) as usize]);
                    }

                    if fft_minus_46[bin_position] > max_neighbor_in_fft_minus_49 {
                        // Ensure that it is a time-domain local minimum
                        let mut max_neighbor_in_other_adjacent_ffts = max_neighbor_in_fft_minus_49;

                        for other_offset in &[
                            -53, -45, 165, 172, 179, 186, 193, 200, 214, 221, 228, 235, 242, 249,
                        ] {
                            let other_fft = &self.spread_fft_outputs
                                [((self.spread_fft_outputs_index as i32 + other_offset) & 255)
                                    as usize];
                            max_neighbor_in_other_adjacent_ffts =
                                max_neighbor_in_other_adjacent_ffts.max(other_fft[bin_position - 1]);
                        }

                        if fft_minus_46[bin_position] > max_neighbor_in_other_adjacent_ffts {
                            // Store the values needed for peak calculation
                            peaks_to_store.push((
                                bin_position,
                                fft_minus_46[bin_position],
                                fft_minus_46[bin_position - 1],
                                fft_minus_46[bin_position + 1],
                            ));
                        }
                    }
                }
            }
        }

        // Now store the peaks (no more borrows of fft arrays)
        for (bin_position, val, val_before, val_after) in peaks_to_store {
            self.store_peak(bin_position, val, val_before, val_after);
        }
    }

    /// Store a detected peak in the signature.
    fn store_peak(&mut self, bin_position: usize, val: f32, val_before: f32, val_after: f32) {
        let fft_pass_number = self.num_spread_ffts_done - 46;

        let peak_magnitude: f32 = val.ln().max(1.0 / 64.0) * 1477.3 + 6144.0;
        let peak_magnitude_before: f32 = val_before.ln().max(1.0 / 64.0) * 1477.3 + 6144.0;
        let peak_magnitude_after: f32 = val_after.ln().max(1.0 / 64.0) * 1477.3 + 6144.0;

        let peak_variation_1: f32 =
            peak_magnitude * 2.0 - peak_magnitude_before - peak_magnitude_after;
        let peak_variation_2: f32 =
            (peak_magnitude_after - peak_magnitude_before) * 32.0 / peak_variation_1;

        let corrected_peak_frequency_bin: u16 =
            ((bin_position as i32 * 64) + (peak_variation_2 as i32)) as u16;

        assert!(peak_variation_1 >= 0.0);

        // Convert back a FFT bin to a frequency, given a 16 KHz sample
        // rate, 1024 useful bins and the multiplication by 64 made before
        // storing the information
        let frequency_hz: f32 =
            corrected_peak_frequency_bin as f32 * (16000.0 / 2.0 / 1024.0 / 64.0);

        // Ignore peaks outside the 250 Hz-5.5 KHz range
        let frequency_band = match frequency_hz as i32 {
            250..=519 => FrequencyBand::_250_520,
            520..=1449 => FrequencyBand::_520_1450,
            1450..=3499 => FrequencyBand::_1450_3500,
            3500..=5500 => FrequencyBand::_3500_5500,
            _ => return,
        };

        self.signature
            .frequency_band_to_sound_peaks
            .entry(frequency_band)
            .or_default()
            .push(FrequencyPeak {
                fft_pass_number,
                peak_magnitude: peak_magnitude as u16,
                corrected_peak_frequency_bin,
            });
    }
}
