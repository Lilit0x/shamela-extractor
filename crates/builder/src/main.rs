use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// reads a single extracted dictionary JSONL and understand the structure
    Explore {
        /// the dictionary file to read
        #[arg(short, long)]
        file: String,
    },
}

#[derive(Debug, Deserialize)]
struct JsonMeta {
    #[serde(rename = "type")]
    _kind: String,
    id: String,
    body_store: String,
}

#[derive(Debug)]
struct BookMetadata {
    id: String,
    title: String,
    author: String,
    publisher: String,
    edition: String,
    volumes: String,
    notes: String,
}

impl BookMetadata {
    fn from_json(s: &str) -> anyhow::Result<Self> {
        let raw: JsonMeta = serde_json::from_str(s)?;

        let mut title = String::new();
        let mut author = String::new();
        let mut publisher = String::new();
        let mut edition = String::new();
        let mut volumes = String::new();
        let mut notes = String::new();

        for line in raw.body_store.split('\r') {
            let line = line.trim();
            if let Some(val) = line.strip_prefix("الكتاب:") {
                title = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("المؤلف:") {
                author = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("الناشر:") {
                publisher = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("الطبعة:") {
                edition = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("عدد الأجزاء:") {
                volumes = val.trim().to_string();
            } else if line.starts_with('[') {
                notes = line.to_string();
            }
        }

        Ok(BookMetadata {
            id: raw.id,
            title,
            author,
            publisher,
            edition,
            volumes,
            notes,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Title {
    #[serde(rename = "type")]
    kind: String,
    id: String,
    body: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    match cli.command {
        Commands::Explore { file } => {
            let path = PathBuf::from(file);
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            let mut lines = reader.lines();

            let metadata: BookMetadata = match lines.next() {
                Some(Ok(line)) => BookMetadata::from_json(&line)?,
                Some(Err(e)) => anyhow::bail!("failed to read first line: {e}"),
                None => anyhow::bail!("empty file"),
            };

            let mut titles = Vec::new();
            let mut pages = 0;

            for l in lines {
                let line = l?;
                if line.contains("\"type\":\"titles\"") {
                    let title: Title = serde_json::from_str(&line)?;
                    titles.push(title);
                } else if line.contains("\"type\":\"pages\"") {
                    pages += 1;
                }
            }

            println!("{metadata:#?}");
            println!("Titles: {}", titles.len());
            println!("Pages: {}", pages);

            for t in titles.iter() {
                println!("  {} | {}", t.id, t.body);
            }
        }
    };
    Ok(())
}
