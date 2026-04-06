## Shamela Extractor
It simply extracts books from a dump of jsonl files which are from Shamela's v4 SQLite DB. As long as you have the 
latest Shamela installed, you should be able to use this tool to get whatever book it is you want.

The v4 Shamela uses Lucene to store books in indexes, so, using a Java extractor to dump the indexes in a JSONL file. The JSONL files are what this tool operates on.

## Extracting Shamela Lucene Indexes to JSONL (one-time). 
**Note**: This section and the relevant Java code was generated using Claude, so, feel free to 
take it with a pinch of salt or recommend better ways.

The Shamela v4 desktop app stores all book content in Lucene 9.x indexes
under `database/store/`. This step dumps those indexes into plain JSONL
files that the Rust CLI can process.

### Requirements

- **JDK 11 or newer** — download from [Adoptium](https://adoptium.net/)
  (Shamela's bundled JRE does not include `javac`)

### Download dependencies

Create a `libs/` folder in your Shamela installation directory and download
these two JARs:

- [lucene-core-9.10.0.jar](https://repo1.maven.org/maven2/org/apache/lucene/lucene-core/9.10.0/lucene-core-9.10.0.jar) (~3.5 MB)
- [lucene-backward-codecs-9.10.0.jar](https://repo1.maven.org/maven2/org/apache/lucene/lucene-backward-codecs/9.10.0/lucene-backward-codecs-9.10.0.jar) (~690 KB)

The backward-codecs JAR is required because Shamela's indexes use the
Lucene 9.5 codec.

### Compile the extractor

Copy `extraction/ShamelaDump.java` to your Shamela installation root, then:
```bash
javac -cp "libs/lucene-core-9.10.0.jar" ShamelaDump.java
```

### Inspect indexes (optional)

Before extracting, you can inspect any index to see its field names and
sample documents:
```bash
java -cp "libs/lucene-core-9.10.0.jar;libs/lucene-backward-codecs-9.10.0.jar;." \
  ShamelaDump --inspect database/store/page
```

On Linux/macOS, use `:` instead of `;` in the classpath.

### Extract all indexes

Create an output directory and extract each index:
```bash
mkdir extracted

# Book catalog (~8,500 documents, completes in seconds)
java -cp "libs/lucene-core-9.10.0.jar;libs/lucene-backward-codecs-9.10.0.jar;." \
  ShamelaDump database/store/book extracted/books.jsonl

# Table of contents (~3.9M documents, a few minutes)
java -cp "libs/lucene-core-9.10.0.jar;libs/lucene-backward-codecs-9.10.0.jar;." \
  ShamelaDump database/store/title extracted/titles.jsonl

# Page content (~7.35M documents, takes 10-20 minutes)
java -Xmx4g -cp "libs/lucene-core-9.10.0.jar;libs/lucene-backward-codecs-9.10.0.jar;." \
  ShamelaDump database/store/page extracted/pages.jsonl
```

The `-Xmx4g` flag gives Java 4GB of heap memory for the large page index.

### Expected output

| File | Documents | Approximate size |
|------|-----------|-----------------|
| `books.jsonl` | ~8,500 | ~30 MB |
| `titles.jsonl` | ~3.9M | ~500 MB |
| `pages.jsonl` | ~7.35M | ~17 GB |

### Index field reference

**books.jsonl** — one line per book:
| Field | Description |
|-------|-------------|
| `id` | Book ID (used to link pages and titles) |
| `body_store` | Book title, author, publisher, edition info (`\r`-delimited) |
| `hint` | Extended description and commentary about the book |

**pages.jsonl** — one line per page:
| Field | Description |
|-------|-------------|
| `id` | Format: `bookId-pageId` (e.g., `1687-5935`) |
| `body` | Main Arabic text content |
| `foot` | Footnotes |
| `comment` | Commentary / annotations |
| `m_body`, `n_body` | Marked/normalized body variants |
| `m_foot`, `n_foot` | Marked/normalized footnote variants |
| `book_key`, `book`, `author`, `group` | Metadata fields (when present) |

**titles.jsonl** — one line per TOC entry:
| Field | Description |
|-------|-------------|
| `id` | Format: `bookId-titleId` (e.g., `1687-42`) |
| `body` | Heading/section title text |


## Extraction 
First clone this repo, get [Rust](https://rust-lang.org/tools/install/), `cd` into `shamela-extractor`.
After the above steps are completed, you just set `SHAMELA_EXTRACTED_DIR` env variable to point to where the jsonl files
are, then you run; `shamela-extractor search --name "لسان العرب"` to search for a single book in order to get its id. Then `shamela-extractor extract --id 1687 --id 7283 --output ./dicts/` to extract multiple books and save them in the `dicts` folder in `cwd`.
