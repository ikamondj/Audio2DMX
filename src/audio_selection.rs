use cpal::traits::{DeviceTrait, HostTrait};
use std::io::{self, Write};

pub enum DeviceSelectionMode {
    DefaultInput,
    DefaultOutput,
    ChooseInteractive,
}

pub fn parse_args() -> DeviceSelectionMode {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "-c") {
        DeviceSelectionMode::ChooseInteractive
    } else if args.iter().any(|a| a == "-o") {
        DeviceSelectionMode::DefaultOutput
    } else {
        // no args, or -i
        DeviceSelectionMode::DefaultInput
    }
}

pub fn choose_audio_device(mode: DeviceSelectionMode) -> cpal::Device {
    let host = cpal::default_host();

    match mode {
        DeviceSelectionMode::DefaultInput => {
            return host.default_input_device()
                .expect("No default input device found");
        }
        DeviceSelectionMode::DefaultOutput => {
            return host.default_output_device()
                .expect("No default output device found");
        }
        DeviceSelectionMode::ChooseInteractive => {
            return choose_interactive(&host);
        }
    }
}

fn choose_interactive(host: &cpal::Host) -> cpal::Device {
    // Collect devices
    let input_devices: Vec<cpal::Device> = host.input_devices()
        .expect("Failed to get input devices")
        .collect();

    let output_devices: Vec<cpal::Device> = host.output_devices()
        .expect("Failed to get output devices")
        .collect();

    // Combine for unified index
    // [0..input.len()) -> inputs
    // next -> outputs
    let mut all_devices: Vec<(String, bool)> = Vec::new(); 
    // (name, is_input)

    for dev in &input_devices {
        all_devices.push((dev.name().unwrap_or("Unknown Input".into()), true));
    }
    for dev in &output_devices {
        all_devices.push((dev.name().unwrap_or("Unknown Output".into()), false));
    }

    println!("Available audio devices:\n");

    println!("Input devices:");
    for (i, (name, is_input)) in all_devices.iter().enumerate() {
        if *is_input {
            let key = device_key(i);
            println!("[{}] {}", key, name);
        }
    }

    println!("\nOutput devices:");
    for (i, (name, is_input)) in all_devices.iter().enumerate() {
        if !*is_input {
            let key = device_key(i);
            println!("[{}] {}", key, name);
        }
    }

    print!("\nSelect a device: ");
    io::stdout().flush().unwrap();

    let mut selection = String::new();
    io::stdin().read_line(&mut selection).unwrap();
    let selection = selection.trim();

    let idx = decode_key(selection)
        .expect("Invalid key; must be 0-9 or a-z");

    if idx >= all_devices.len() {
        panic!("Key out of range");
    }

    if all_devices[idx].1 {
        return input_devices[idx].clone();
    } else {
        let out_idx = idx - input_devices.len();
        return output_devices[out_idx].clone();
    }
}

/// Map integer index -> display key
fn device_key(idx: usize) -> char {
    if idx < 10 {
        std::char::from_digit(idx as u32, 10).unwrap()
    } else {
        ((idx - 10) as u8 + b'a') as char
    }
}

/// Convert user-typed key back to index
fn decode_key(s: &str) -> Option<usize> {
    let c = s.chars().next()?;

    if c.is_ascii_digit() {
        return c.to_digit(10).map(|d| d as usize);
    }

    if c.is_ascii_lowercase() {
        let offset = (c as u8) - b'a';
        return Some(10 + offset as usize);
    }

    None
}
