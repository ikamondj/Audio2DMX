use cpal::traits::{DeviceTrait, StreamTrait};
use realfft::RealFftPlanner;
use std::io::{self, Write};
use std::sync::mpsc::{self};
use std::{
    time::Duration,
};


use std::collections::HashMap;
use crate::state::AppState;
use crate::effects::EffectSuite;
use crate::dmx::send_dmx_frame;


pub async fn audio_loop(state: AppState, glob_effects: HashMap<String, EffectSuite>, ord_effects: Vec<EffectSuite>, num_bins:usize) {
    let device = state.device.clone();   // <── use the selected device ONCE

    loop {
        //
        // 1. Read ONLY fft_size from the map
        //
        let fft_size: usize = num_bins as usize;

        println!("[Audio] Opening device {:?} with fft={}", device.name(), fft_size);

        //
        // 2. Use THIS DEVICE only
        //
        let config = device.default_input_config().unwrap();
        println!("[Audio] Actual input config: {:?}", config);

        //
        // 3. Channel for audio samples
        //
        let (tx, rx) = mpsc::channel::<f32>();

        // Build the stream
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _| {
                    for &s in data {
                        let _ = tx.send(s);
                    }
                },
                move |err| eprintln!("Stream error: {err}"),
                None,
            ),
            fmt => panic!("Unsupported sample format {:?}", fmt),
        }.expect("Failed to build input stream");

        stream.play().expect("Failed to start audio stream");

        //
        // 4. Setup FFT
        //
        let mut planner = RealFftPlanner::<f32>::new();
        let r2c = planner.plan_fft_forward(fft_size);

        let mut input = r2c.make_input_vec();
        let mut spectrum = r2c.make_output_vec();

        println!("[Audio] Stream started. Entering processing loop...");

        //
        // 5. Inner audio loop
        //
        loop {
            //
            // A. Check ONLY if FFT size changed
            //
            {
                let map = state.store.read().unwrap();
                let new_fft = map.get("fft_size")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(fft_size as u64) as usize;

                if new_fft != fft_size {
                    println!("[Audio] FFT size changed → restarting stream...");
                    break;
                }
            }

            //
            // B. Fill FFT buffer
            //
            for i in 0..fft_size {
                input[i] = match rx.recv() {
                    Ok(s) => s,
                    Err(_) => {
                        println!("[Audio] Audio stream ended unexpectedly.");
                        continue;
                    }
                };
            }

            //
            // C. FFT
            //
            r2c.process(&mut input, &mut spectrum).unwrap();

            //
            // D. Convert to magnitudes
            //
            let magnitudes: Vec<f32> = spectrum.iter().map(|c| c.norm()).collect();

            let n = magnitudes.len();
            let mut weights = Vec::with_capacity(n);

            for i in 0..n {
                let freq_ratio = i as f32 / n as f32;  // 0.0 = bass, 1.0 = high
                let w = 0.3 + 0.7 * freq_ratio;        // linear tilt upward
                weights.push(w);
            }

            let regularized: Vec<f32> = magnitudes
                .iter()
                .zip(weights.iter())
                .map(|(&m, &w)| m * w)
                .collect();

            let transformed: Vec<f32> = regularized
                .iter()
                .map(|&x| {
                    let y = 1.0 - (4.0_f32).powf(-x);
                    y.clamp(0.0, 1.0)
                })
                .collect();

            let mut cumulative = Vec::with_capacity(transformed.len());
            let mut running_sum = 0.0_f32;

            for &val in &transformed {
                running_sum += val;
                cumulative.push(running_sum);
            }

            let mut logfft = String::new();
            for x in transformed {
                if x < 0.1 {
                    logfft.push('.');
                } else if x < 0.25 {
                    logfft.push(',');
                } else if x < 0.5 {
                    logfft.push(':');
                } else if x < 0.7 {
                    logfft.push('i');
                } else if x < 0.85 {
                    logfft.push('I');
                } else {
                    logfft.push('|');
                }
            }

            print!("\r{}", logfft);

            let map = state.store.read().unwrap();

            if let Some(effect_val) = map.get("effect") {

                // Case 1: string effect key, like "pulse" or "wave"
                if let Some(effect_str) = effect_val.as_str() {
                    if let Some(suite) = glob_effects.get(effect_str) {
                        let frame_json = suite.process(&cumulative);
                        let _ = send_dmx_frame(&frame_json).await;
                    }

                // Case 2: numeric effect index, like 0, 1, 2...
                } else if let Some(idx) = effect_val.as_u64() {
                    let idx = idx as usize;

                    if idx < ord_effects.len() {
                        if let Some(suite) = ord_effects.get(idx) {
                            let frame_json = suite.process(&cumulative);
                            let _ = send_dmx_frame(&frame_json).await;
                        }
                    }
                }
            }           
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}