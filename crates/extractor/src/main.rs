use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process::exit,
    time::Instant,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to extracted shamela artifacts
    #[arg(short, long, env = "SHAMELA_EXTRACTED_DIR")]
    extract_dir: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Search for a book using a partail token from the name
    Search {
        ///
        #[arg(short, long)]
        name: String,
    },
    Extract {
        /// id of the book to extract, you can it thru search
        #[arg(short, long)]
        id: Vec<String>,
        /// directory to store the extracted artifact
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct BookRecord {
    id: String,
    body_store: Option<String>,
    hint: Option<String>,
    body: Option<String>,
    author: Option<String>,
    book: Option<String>,
    date: Option<String>,
    group: Option<String>,
}

// TODO: change to Extractor struct and make the functions methods
//

fn find_books_by_id(
    ids: &[String],
    shamela_extracted_path: &Path,
) -> Result<HashMap<String, BookRecord>> {
    let books_path = shamela_extracted_path.join("books.jsonl");
    let file = File::open(books_path)?;
    let reader = BufReader::new(file);

    let ids2: HashSet<&str> = ids.iter().map(|id| id.as_str()).collect();
    let mut book_records = HashMap::new();

    for l in reader.lines() {
        let line = l?;
        let record: BookRecord =
            serde_json::from_str(&line).with_context(|| "deserialized fail")?;
        if ids2.contains(record.id.as_str()) {
            book_records.insert(record.id.clone(), record);
        }
        if book_records.len() == ids.len() {
            break;
        }
    }

    Ok(book_records)
}

/// for now it returns the id
fn search_for_book(token: &str, shamela_extracted_path: &Path) -> Result<Vec<BookRecord>> {
    let books_path = shamela_extracted_path.join("books.jsonl");
    let file = File::open(books_path)?;
    let reader = BufReader::new(file);

    let mut results = Vec::new();

    for l in reader.lines() {
        let line = l?;
        if line.contains(token) {
            let record: BookRecord = serde_json::from_str(&line)?;
            results.push(record)
        }
    }

    Ok(results)
}

fn scan_and_write(
    file_path: &Path,
    prefixes: &[(String, String)],
    writers: &mut HashMap<String, BufWriter<File>>,
    label: &str,
) -> Result<usize> {
    let file_size = std::fs::metadata(file_path)?.len();
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut count = 0;
    let mut total_lines = 0u64;
    let mut bytes_read = 0u64;
    loop {
        buf.clear();
        let bytes = reader.read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }
        total_lines += 1;
        bytes_read += bytes as u64;

        if total_lines % 500_000 == 0 {
            let pct = 100.0 * bytes_read as f64 / file_size as f64;
            eprint!("\r  scanning {}: {:.1}% ({} matches)", label, pct, count);
        }

        for (book_id, prefix) in prefixes {
            if buf.contains(prefix.as_str()) {
                if let Some(w) = writers.get_mut(book_id) {
                    write!(w, "{{\"type\":\"{}\",{}", label, &buf[1..])?;
                }
                count += 1;
                break;
            }
        }
    }
    eprintln!(
        "\r  {}: done — {} matches in {} lines              ",
        label, count, total_lines
    );
    Ok(count)
}

fn extract_books(
    ids: Vec<String>,
    shamela_extracted_path: &Path,
    output_path: &Path,
) -> Result<usize> {
    let mut prefixes = Vec::new();
    let mut writers: HashMap<String, BufWriter<File>> = HashMap::new();

    let metadatas = find_books_by_id(&ids, shamela_extracted_path)?;
    for (id, metadata) in metadatas {
        prefixes.push((id.clone(), format!("\"id\":\"{id}-")));

        let result_path = output_path.join(format!("{id}.jsonl"));
        let parent_path = result_path.parent().unwrap();
        std::fs::create_dir_all(parent_path).unwrap();

        let mut writer = BufWriter::new(File::create(&result_path).context(format!(
            "creating output file: {:#?}",
            result_path.display()
        ))?);
        writeln!(
            writer,
            "{{\"type\":\"meta\",{}",
            &serde_json::to_string(&metadata)?[1..]
        )?;
        writers.insert(id.to_string(), writer);
    }

    let _ = scan_and_write(
        &shamela_extracted_path.join("titles.jsonl"),
        &prefixes,
        &mut writers,
        "titles",
    )?;
    let pages = scan_and_write(
        &shamela_extracted_path.join("pages.jsonl"),
        &prefixes,
        &mut writers,
        "pages",
    )?;

    Ok(pages as usize)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let shamela_extraction_dir = PathBuf::from(args.extract_dir);
    ["books", "pages", "titles"].iter().for_each(|file| {
        let path = shamela_extraction_dir.join(format!("{file}.jsonl"));
        if !path.exists() {
            eprintln!(
                "{file}.jsonl does not exist in {}",
                shamela_extraction_dir.to_str().unwrap()
            );
            exit(1);
        }
    });

    if let Some(cmd) = args.command {
        match cmd {
            Commands::Search { name } => {
                search_for_book(&name, &shamela_extraction_dir)
                    .context(format!("searching for book with token: {name}"))?
                    .iter()
                    .for_each(|r| {
                        let title = r
                            .body_store
                            .as_ref()
                            .and_then(|b| b.split('\r').next())
                            .unwrap_or("unknown");
                        println!("  id: {}\n  title: {}\n", r.id, title);
                    });
            }
            Commands::Extract { id, output } => {
                let output_dir = PathBuf::from(output.unwrap_or_else(|| ".".to_string()));
                let start = Instant::now();
                let pages = extract_books(id, &shamela_extraction_dir, &output_dir)?;
                println!(
                    "extracted {} pages in {:.1}s",
                    pages,
                    start.elapsed().as_secs_f64()
                );
            }
        }
    };

    Ok(())
}
