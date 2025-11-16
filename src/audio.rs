use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use realfft::RealFftPlanner;
use std::sync::mpsc::{self, Receiver};
use std::{
    time::Duration,
};

use crate::state::AppState;



pub async fn audio_loop(state: AppState) {
    loop {
        //
        // 1. Read initial config before opening device
        //
        let (device_name, fft_size) = {
            let map = state.store.read().unwrap();
            let dev = map.get("audio_device")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string();

            let fft = map.get("fft_size")
                .and_then(|v| v.as_u64())
                .unwrap_or(512) as usize;

            (dev, fft)
        };

        println!("\n[Audio] Opening device '{device_name}' with fft={fft_size}");

        //
        // 2. Open audio input device
        //
        let host = cpal::default_host();

        let device = if device_name == "default" {
            host.default_input_device()
                .expect("No default input device")
        } else {
            host.devices()
                .unwrap()
                .find(|d| d.name().unwrap() == device_name)
                .unwrap_or_else(|| {
                    panic!("Audio device '{device_name}' not found")
                })
        };

        let config = device.default_input_config().unwrap();
        println!("[Audio] Actual input config: {:?}", config);

        //
        // 3. Channel between CPAL callback and FFT thread
        //
        let (tx, rx) = std::sync::mpsc::channel::<f32>();

        // Build input stream
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
            other => panic!("Unsupported audio format {:?}", other),
        }
        .expect("Failed to build input stream");

        stream.play().expect("Failed to start audio stream");

        //
        // 4. Setup FFT
        //
        use realfft::RealFftPlanner;
        let mut planner = RealFftPlanner::<f32>::new();
        let r2c = planner.plan_fft_forward(fft_size);

        let mut input = r2c.make_input_vec();
        let mut spectrum = r2c.make_output_vec();

        println!("[Audio] Stream started. Entering processing loop...");

        //
        // 5. AUDIO PROCESS INNER LOOP
        //
        loop {
            //
            // A. Check for device or FFT-size change
            //
            {
                let map = state.store.read().unwrap();

                let new_dev = map.get("audio_device")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default");

                let new_fft = map.get("fft_size")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(fft_size as u64) as usize;

                if new_dev != device_name || new_fft != fft_size {
                    println!("[Audio] Configuration changed, restarting audio stream...");
                    break; // break inner loop → restart device
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
            // C. Run FFT
            //
            r2c.process(&mut input, &mut spectrum).unwrap();

            //
            // D. Convert FFT to magnitudes
            //
            let magnitudes: Vec<f32> = spectrum.iter().map(|c| c.norm()).collect();

            // Example: print first few bins for debugging
            println!("bins: {:?}", &magnitudes[..8.min(magnitudes.len())]);
        }

        // Wait a moment before recreating stream
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}