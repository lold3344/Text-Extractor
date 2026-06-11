use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::Mutex;
use scraper::{Html, Selector};
use rayon::prelude::*;

const ARIA_PATH: &str = r"C:\Users\lold\Documents\GitHub\ARIA\data base\DataBase.txt";
const DESKTOP_FILENAME: &str = "DataBase.txt";
const MAX_URLS: usize = 500;

fn is_russian(text: &str) -> bool {
    let letters: Vec<char> = text.chars().filter(|c| c.is_alphabetic()).collect();
    if letters.len() < 10 {
        return false;
    }
    let cyrillic = letters.iter().filter(|&&c| ('\u{0400}'..='\u{04FF}').contains(&c)).count();
    cyrillic as f32 / letters.len() as f32 > 0.5
}

fn clean_text(text: &str) -> String {
    let mut result = String::new();
    let mut depth = 0usize;
    for ch in text.chars() {
        match ch {
            '[' => depth += 1,
            ']' => { if depth > 0 { depth -= 1; } }
            _ if depth == 0 => result.push(ch),
            _ => {}
        }
    }
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn collect_text(html: &str, tag: &str) -> String {
    let document = Html::parse_document(html);
    let containers = [
        "#mw-content-text .mw-parser-output",
        "#mw-content-text",
        "article",
        "main",
        "[role=\"main\"]",
        ".post-content",
        ".article-body",
        ".entry-content",
        "#content",
        "body",
    ];

    let Ok(tag_sel) = Selector::parse(tag) else { return String::new() };

    for sel_str in &containers {
        let Ok(sel) = Selector::parse(sel_str) else { continue };
        let Some(container) = document.select(&sel).next() else { continue };

        let mut text = String::new();
        for el in container.select(&tag_sel) {
            let raw = el.text().collect::<Vec<_>>().join(" ");
            let cleaned = clean_text(&raw);
            if cleaned.len() > 40 && is_russian(&cleaned) {
                text.push_str(&cleaned);
                text.push('\n');
            }
        }

        if !text.is_empty() {
            return text;
        }
    }

    String::new()
}

fn extract_paragraphs(html: &str) -> String {
    // try tags in order: paragraphs first, then divs, then table cells
    for tag in &["p", "div", "td", "li"] {
        let text = collect_text(html, tag);
        if !text.is_empty() {
            return text;
        }
    }
    String::new()
}

fn fetch_text(url: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;

    let html = client.get(url).send().map_err(|e| e.to_string())?.text().map_err(|e| e.to_string())?;
    Ok(extract_paragraphs(&html))
}

fn resolve_output_path() -> String {
    if std::path::Path::new(ARIA_PATH).parent().map_or(false, |p| p.exists()) {
        ARIA_PATH.to_string()
    } else {
        let desktop = dirs::desktop_dir()
            .unwrap_or_else(|| std::path::PathBuf::from(r"C:\Users\lold\Desktop"));
        desktop.join(DESKTOP_FILENAME).to_string_lossy().to_string()
    }
}

fn save_text(text: &str, path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    file.write_all(text.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(text.chars().count())
}

fn process_urls(urls: Vec<String>) {
    let total = urls.len();
    let output_path = resolve_output_path();
    println!("Processing {} URLs...", total);
    println!("Output: {}\n", output_path);

    let file_mutex = Mutex::new(());
    let counter = Mutex::new(0usize);

    urls.par_iter().for_each(|url| {
        print!("Fetching: {}\n", url);
        io::stdout().flush().unwrap();

        match fetch_text(url) {
            Ok(text) => {
                if text.is_empty() {
                    println!("[SKIP] No Russian text: {}", url);
                } else {
                    let chars = text.chars().count();
                    let _lock = file_mutex.lock().unwrap();
                    match save_text(&text, &output_path) {
                        Ok(_) => {
                            let mut count = counter.lock().unwrap();
                            *count += 1;
                            println!("[OK] {} chars - {}", chars, url);
                        }
                        Err(e) => println!("[ERROR] Save failed for {}: {}", url, e),
                    }
                }
            }
            Err(e) => println!("[ERROR] Fetch failed for {}: {}", url, e),
        }
    });

    let saved = *counter.lock().unwrap();
    println!("\nDone: {}/{} pages saved to DataBase.txt", saved, total);
}

fn main() {
    let output_path = resolve_output_path();
    println!("=== Text Extractor for ARIA ===");
    println!("Output: {}", output_path);
    println!("Max URLs per batch: {}\n", MAX_URLS);
    println!("Paste up to {} URLs (one per line).", MAX_URLS);
    println!("When done - type 'go' and press Enter.\n");

    let mut urls: Vec<String> = Vec::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let line = input.trim().to_string();

        if line == "exit" {
            println!("Goodbye.");
            break;
        }

        if line == "go" {
            if urls.is_empty() {
                println!("No URLs entered.\n");
                continue;
            }
            process_urls(urls.clone());
            urls.clear();
            println!("\nPaste more URLs or type 'exit'.\n");
            continue;
        }

        if line.is_empty() {
            continue;
        }

        if !line.starts_with("http://") && !line.starts_with("https://") {
            println!("Skipping (not a URL): {}\n", line);
            continue;
        }

        if urls.len() >= MAX_URLS {
            println!("Limit of {} URLs reached. Type 'go' to process.\n", MAX_URLS);
            continue;
        }

        urls.push(line.clone());
        println!("Added [{}/{}]: {}", urls.len(), MAX_URLS, line);
    }
}
