use std::fs::OpenOptions;
use std::io::{self, Write};
use scraper::{Html, Selector};

const OUTPUT_PATH: &str = r"C:\Users\lold\Documents\GitHub\ARIA\data base\DataBase.txt";

fn fetch_text(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Fetching: {}", url);

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let html = client.get(url).send()?.text()?;
    let document = Html::parse_document(&html);

    // remove scripts and styles
    let mut text = String::new();
    let selector = Selector::parse("p, h1, h2, h3, h4, li").unwrap();

    for element in document.select(&selector) {
        let t = element.text().collect::<Vec<_>>().join(" ");
        let t = t.trim().to_string();
        if t.len() > 20 {
            text.push_str(&t);
            text.push('\n');
        }
    }

    Ok(text)
}

fn save_text(text: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(OUTPUT_PATH)?;

    file.write_all(text.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(text.len())
}

fn main() {
    println!("=== Text Extractor for ARIA ===");
    println!("Output: {}\n", OUTPUT_PATH);
    println!("Enter URL (or 'exit' to quit):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let url = input.trim();

        if url == "exit" {
            println!("Goodbye.");
            break;
        }

        if url.is_empty() {
            continue;
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            println!("Error: URL must start with http:// or https://\n");
            continue;
        }

        match fetch_text(url) {
            Ok(text) => {
                if text.is_empty() {
                    println!("No text found on this page.\n");
                    continue;
                }
                match save_text(&text) {
                    Ok(bytes) => println!("Saved {} characters to DataBase.txt\n", bytes),
                    Err(e) => println!("Save error: {}\n", e),
                }
            }
            Err(e) => println!("Fetch error: {}\n", e),
        }
    }
}
