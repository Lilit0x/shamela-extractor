use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
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

    /// id of the book to search for
    #[arg(short, long)]
    id: Option<String>,

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
        id: String,
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

fn find_book_by_id(id: &str, shamela_extracted_path: &Path) -> Result<Option<BookRecord>> {
    let books_path = shamela_extracted_path.join("books.jsonl");
    let file = File::open(books_path)?;
    let reader = BufReader::new(file);

    for l in reader.lines() {
        let line = l?;
        let record: BookRecord = serde_json::from_str(&line)
            .with_context(|| "failed to deserialize book record".to_string())?;
        if record.id == id {
            return Ok(Some(record));
        }
    }

    Ok(None)
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
    prefix: &str,
    writer: &mut BufWriter<File>,
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

        if buf.contains(prefix) {
            write!(writer, "{}", buf)?;
            count += 1;
        }
    }
    eprintln!(
        "\r  {}: done — {} matches in {} lines              ",
        label, count, total_lines
    );
    Ok(count)
}

fn extract_book(id: &str, shamela_extracted_path: &Path, output_path: &Path) -> Result<usize> {
    let metadata = find_book_by_id(id, shamela_extracted_path)?
        .with_context(|| format!("no book found with id: {id}"))?;

    // this will enable us to check if a line contains this, so we won't need to deserialize all lines
    let prefix = format!("\"id\":\"{id}-");
    let result_path = output_path.join(format!("{id}.jsonl"));

    let parent_path = result_path.parent().unwrap();
    std::fs::create_dir_all(parent_path).unwrap();

    let mut writer = BufWriter::new(File::create(&result_path).context(format!(
        "creating output file: {:#?}",
        result_path.display()
    ))?);
    writeln!(writer, "{}", serde_json::to_string(&metadata)?)?;

    let _ = scan_and_write(
        &shamela_extracted_path.join("titles.jsonl"),
        &prefix,
        &mut writer,
        "titles",
    )?;
    let pages = scan_and_write(
        &shamela_extracted_path.join("pages.jsonl"),
        &prefix,
        &mut writer,
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

    if let Some(id) = args.id {
        let res =
            find_book_by_id(&id, &shamela_extraction_dir).context(format!("finding book: {id}"))?;
        println!("{res:#?}")
    }

    if let Some(cmd) = args.command {
        match cmd {
            Commands::Search { name } => {
                search_for_book(&name, &shamela_extraction_dir)
                    .context(format!("searching for book with token: {name}"))?
                    .iter()
                    .for_each(|r| {
                        let text = r
                            .body_store
                            .as_ref()
                            // i used char here because arabic chars are 2 byte strings, i could've used .truncate(),
                            // but errrors happened.
                            .map(|b| b.chars().take(80).collect::<String>())
                            .unwrap_or_default();
                        println!("[{}] {}", r.id, text)
                    });
            }
            Commands::Extract { id, output } => {
                let output_dir = PathBuf::from(output.unwrap_or_else(|| ".".to_string()));
                let start = Instant::now();
                let pages = extract_book(&id, &shamela_extraction_dir, &output_dir)
                    .context(format!("extracting book; {id}"))?;
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
