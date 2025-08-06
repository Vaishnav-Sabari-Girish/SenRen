use clap::{Parser, Subcommand};
use std::process::Command;
use std::{thread, time::Duration};
use std::io::{Write, BufWriter};
use std::fs::{File, OpenOptions};
use std::path::Path;
use rand::prelude::*;

// For easy updating if you ever want to move the temp devices
const WRITER_PORT: &str = "/tmp/ttyV1";
const MONITOR_PORT: &str = "/tmp/ttyV0";

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Run with virtual ports (auto-creates /tmp/ttyV0 & /tmp/ttyV1 and streams fake data)
    #[arg(long, default_value_t = false)]
    virtual_mode: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start serial monitor with ComChan
    Monitor {
        #[arg(default_value = MONITOR_PORT)]
        port: String,
        #[arg(default_value_t = 115200)]
        baud: u32,
        #[arg(long, default_value_t = true)]
        plot: bool,
    },
    /// Collect dummy scan and export to CSV
    Scan,
    /// Plot the scan data using Python
    Plot,
}

fn collect_dummy_scan() -> Vec<(f64, f64)> {
    let mut rng = rand::rng();
    // Simulates a sinusoidal pattern (like an obstacle) with random noise
    (0..360)
        .map(|angle| {
            let base_dist = 1000.0 + 300.0 * ((angle as f64).to_radians().sin() * 2.0).sin();
            let noise: f64 = rng.random_range(-50.0..50.0);
            (angle as f64, (base_dist + noise).max(0.0))
        })
        .collect()
}

fn export_scan(data: &[(f64, f64)], path: &str) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "angle,distance")?;
    for (angle, dist) in data {
        writeln!(writer, "{},{}", angle, dist)?;
    }
    Ok(())
}

fn launch_comchan(port: &str, baud: u32, plot: bool) {
    let mut cmd = Command::new("comchan");
    cmd.arg("--port").arg(port)
        .arg("--baud").arg(baud.to_string());
    if plot {
        cmd.arg("--plot");
    }
    let _ = cmd.spawn();
}

fn launch_socat_virtual_ports() -> std::process::Child {
    Command::new("socat")
        .arg("-d")
        .arg("-d")
        .arg("pty,raw,echo=0,link=/tmp/ttyV0")
        .arg("pty,raw,echo=0,link=/tmp/ttyV1")
        .spawn()
        .expect("Failed to start socat (make sure socat is installed and writable)")
}

fn wait_for_virtual_port(path: &str, timeout_secs: u64) -> bool {
    for _ in 0..(timeout_secs * 10) {
        if Path::new(path).exists() {
            return true;
        }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

fn write_dummy_lidar_data() {
    let mut rng = rand::rng();
    // Wait until /tmp/ttyV1 is created (max 5 seconds)
    if !wait_for_virtual_port(WRITER_PORT, 5) {
        eprintln!("Timeout: {WRITER_PORT} was not created (is socat running?)");
        return;
    }
    let mut file = match OpenOptions::new().write(true).open(WRITER_PORT) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open {WRITER_PORT}: {:?}", e);
            return;
        }
    };
    loop {
        for angle in 0..360 {
            let base_dist = 1000.0 + 300.0 * ((angle as f64).to_radians().sin() * 2.0).sin();
            let noise: f64 = rng.random_range(-50.0..50.0);
            let dist = (base_dist + noise).max(0.0);
            let quality = 15;
            let line = format!("{},{},{}\n", angle, dist, quality);
            if let Err(e) = file.write_all(line.as_bytes()) {
                eprintln!("Failed to write: {:?}", e);
                break;
            }
            let _ = file.flush();
            thread::sleep(Duration::from_millis(10));
        }
    }
}

fn main() {
    let cli = Cli::parse();
    if cli.virtual_mode {
        println!("Starting socat to create {MONITOR_PORT} <-> {WRITER_PORT}...");
        let mut socat_child = launch_socat_virtual_ports();

        let writer_handle = thread::spawn(write_dummy_lidar_data);

        println!("\nVirtual serial ports CREATED:");
        println!("  Writer   --> {WRITER_PORT}");
        println!("  ComChan <-- {MONITOR_PORT}");
        println!("In a new terminal, run:");
        println!("  comchan --port {MONITOR_PORT} --baud 115200 --plot");
        println!("Ctrl+C to stop everything.\n");

        let _ = writer_handle.join();
        let _ = socat_child.kill();
        return;
    }

    match &cli.command {
        Some(Commands::Monitor { port, baud, plot }) => {
            println!("Launching ComChan on port {} at {} baud...", port, baud);
            launch_comchan(port, *baud, *plot);
        }
        Some(Commands::Scan) => {
            println!("Exporting dummy scan to scan_data.csv...");
            let scan = collect_dummy_scan();
            export_scan(&scan, "scan_data.csv").expect("Failed to write CSV");
        }
        Some(Commands::Plot) => {
            println!("Plotting scan_data.csv with Python...");
            let _ = Command::new("python")
                .arg("plot_scan.py")
                .arg("scan_data.csv")
                .spawn()
                .expect("Failed to run Python plot");
        }
        None => {
            println!("No command given. Use --help for usage info.");
        }
    }
}
