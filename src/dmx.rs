use serde_json::{
json,
Value
};
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;
use reqwest::Client;
use anyhow::{Result, anyhow};

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

async fn install_ola() -> Result<(), Box<dyn std::error::Error>> {
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

async fn install_ola_linux() -> Result<(), Box<dyn std::error::Error>> {
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

async fn install_ola_macos() -> Result<(), Box<dyn std::error::Error>> {
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




pub async fn send_dmx_frame(value: &Value) -> Result<()> {
    let channels_val = value
        .get("channels")
        .ok_or_else(|| anyhow!("missing 'channels' field in DMX frame"))?;

    let payload = json!({
        "u": 1,
        "d": channels_val
    });

    let client = Client::new();

    let resp = client
        .post("http://localhost:9090/set_dmx")
        .json(&payload)
        .send()
        .await?;   // works now

    if !resp.status().is_success() {
        return Err(anyhow!("OLA returned HTTP status {}", resp.status()));
    }

    Ok(())
}
