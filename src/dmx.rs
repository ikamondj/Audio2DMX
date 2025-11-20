use serde_json::{
Value
};
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;
use std::net::UdpSocket;
use std::io::Result;

pub async fn spawn_olad() {
    //
    // 0. Check if olad exists
    //
    let olad_exists = Command::new("which")
        .arg("olad")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false);

    if !olad_exists {
        eprintln!("[OLA] olad not found — installing OLA…");

        install_ola().await.expect("[OLA] failed to install OLA");
    } else {
        println!("[OLA] olad already installed.");
    }

    //
    // 1. Spawn olad in foreground mode
    //
    let mut child = Command::new("olad")
        .arg("-f")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start olad");

    println!("[OLA] olad spawned");

    //
    // 2. Async log drainers so child doesn't block
    //
    if let Some(stdout) = child.stdout.take() {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                println!("[OLA][stdout] {}", line);
            }
        });
    }

    if let Some(stderr) = child.stderr.take() {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                eprintln!("[OLA][stderr] {}", line);
            }
        });
    }

    //
    // 3. Supervisor loop (auto-restarts olad)
    //
    tokio::spawn(async move {
        loop {
            match child.wait().await {
                Ok(status) => {
                    eprintln!("[OLA] olad exited: {}", status);
                }
                Err(e) => {
                    eprintln!("[OLA] error waiting for olad: {:?}", e);
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            println!("[OLA] restarting olad...");

            child = Command::new("olad")
                .arg("-f")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to restart olad");
        }
    });
}

async fn install_ola() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;

    match os {
        "linux" => install_ola_linux().await?,
        "macos" => install_ola_macos().await?,
        _ => {
            return Err(format!("Unsupported OS '{}'. Install OLA manually.", os).into());
        }
    }

    Ok(())
}

async fn install_ola_linux() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    // Try apt first (Debian, Ubuntu, Raspberry Pi OS)
    let is_apt = Command::new("which")
        .arg("apt")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?
        .success();

    if is_apt {
        println!("[OLA] Installing OLA via apt...");
        Command::new("sudo")
            .arg("apt")
            .arg("update")
            .status()
            .await?;
        Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg("ola")
            .status()
            .await?;
        return Ok(());
    }

    // Try pacman (Arch Linux)
    let is_pacman = Command::new("which")
        .arg("pacman")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?
        .success();

    if is_pacman {
        println!("[OLA] Installing OLA via pacman...");
        Command::new("sudo")
            .arg("pacman")
            .arg("-Sy")
            .arg("ola")
            .status()
            .await?;
        return Ok(());
    }

    Err("Linux detected but no known package manager found for installing OLA".into())
}

async fn install_ola_macos() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    println!("[OLA] Installing OLA via Homebrew…");

    let brew_exists = Command::new("which")
        .arg("brew")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?
        .success();

    if !brew_exists {
        return Err("Homebrew not found. Install brew first: https://brew.sh".into());
    }

    Command::new("brew")
        .arg("install")
        .arg("ola")
        .status()
        .await?;

    Ok(())
}




pub fn send_artnet_dmx(universe: u16, channels: &[u8]) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Art-Net packet
    let mut packet = Vec::new();
    packet.extend_from_slice(b"Art-Net\0");            // ID
    packet.extend_from_slice(&0x0050u16.to_le_bytes()); // OpOutput (little endian)
    packet.extend_from_slice(&0x000eu16.to_be_bytes()); // ProtVerHi=0, ProtVerLo=14
    packet.push(0);                                     // Sequence
    packet.push(0);                                     // Physical
    packet.extend_from_slice(&universe.to_le_bytes());  // Universe (little endian)
    packet.extend_from_slice(&(channels.len() as u16).to_be_bytes()); // Data length (big endian)
    packet.extend_from_slice(channels);                 // DMX Data

    socket.send_to(&packet, "127.0.0.1:6454")?;
    Ok(())
}



    

pub async fn send_dmx_frame(value: &Value) -> Result<()> {
    // 1. Extract channels object
    let Some(chobj) = value.get("channels") else {
        eprintln!("[DMX ERROR] send_dmx_frame: missing 'channels' in JSON");
        return Ok(()); // Don't crash audio thread
    };

    let Some(map) = chobj.as_object() else {
        eprintln!("[DMX ERROR] send_dmx_frame: channels is not an object");
        return Ok(());
    };

    // 2. Create a full 512-channel DMX buffer
    let mut dmx = vec![0u8; 512];

    // 3. Fill DMX array from the JSON object
    for (ch_str, v) in map.iter() {
        if let Ok(ch) = ch_str.parse::<usize>() {
            if ch >= 1 && ch <= 512 {
                if let Some(val) = v.as_u64() {
                    dmx[ch - 1] = val.min(255) as u8;
                }
            }
        }
    }

    // 4. Send via ArtNet (universe 0)
    // If you want to expose universe selection later, change this argument.
    send_artnet_dmx(0, &dmx)?;

    Ok(())
}
