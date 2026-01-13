use clap::{Parser, Subcommand};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "devlog")]
#[command(about = "A simple progress logger for developers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new log entry
    Add { message: String },
    /// List all entries
    List,
}

#[derive(Serialize, Deserialize)]
struct LogEntry {
    timestamp: String,
    message: String,
}

fn get_log_file() -> PathBuf {
    let mut path = std::env::current_dir().expect("Could not find home directory");
    path.push(".devlog.json");
    path
}

fn load_entries() -> Vec<LogEntry> {
    let path = get_log_file();
    if path.exists() {
        let data = fs::read_to_string(&path).expect("Could not read log file");
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_entries(entries: &Vec<LogEntry>) {
    let path = get_log_file();
    let data = serde_json::to_string_pretty(entries).expect("Could not serialize entries");
    fs::write(path, data).expect("Could not write log file");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { message } => {
            let mut entries = load_entries();
            let entry = LogEntry {
                timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                message: message.clone(),
            };
            entries.push(entry);
            save_entries(&entries);
            println!("✓ Logged: {}", message);
        }
        Commands::List => {
            let entries = load_entries();
            if entries.is_empty() {
                println!("No entries yet. Add one with: devlog add \"your message\"");
            } else {
                for entry in entries {
                    println!("[{}] {}", entry.timestamp, entry.message);
                }
            }
        }
    }
}
