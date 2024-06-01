use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use url::{Url, ParseError};

fn main() -> Result<(), Box<dyn Error>> {
    // Prompt the user for a URL
    print!("Please enter the URL of the website to analyze (e.g., https://example.com): ");
    io::stdout().flush()?;
    
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input)?;
    let url_input = url_input.trim();

    // Parse the base URL
    let base_url = Url::parse(url_input)?;
    let base_domain = base_url.domain().ok_or("Invalid URL")?.to_string();

    // Create a reqwest client
    let client = Client::new();

    // Set up the structures for crawling
    let mut visited: HashSet<String> = HashSet::new();
    let mut to_visit: VecDeque<String> = VecDeque::new();
    let mut all_links: HashSet<String> = HashSet::new();

    to_visit.push_back(base_url.to_string());

    // File to store all links
    let output_file = "links.txt";
    let file = File::create(output_file)?;
    let mut file_writer = BufWriter::new(file);

    while let Some(current_url) = to_visit.pop_front() {
        if visited.contains(&current_url) {
            continue;
        }
        
        println!("Visiting: {}", current_url);
        writeln!(file_writer, "{}", current_url)?;

        visited.insert(current_url.clone());

        // Fetch the HTML content of the current page
        let response = match client.get(&current_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send() {
                Ok(res) => res,
                Err(_) => continue,
            };
        
        let response_text = match response.text() {
            Ok(text) => text,
            Err(_) => continue,
        };

        // Parse the HTML
        let document = Html::parse_document(&response_text);
        
        // Select all anchor tags
        let link_selector = Selector::parse("a").unwrap();
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                let link_url = base_url.join(href).unwrap_or_else(|_| base_url.clone());

                if link_url.domain() == Some(base_domain.as_str()) {
                    let link_url_str = link_url.to_string();
                    if !visited.contains(&link_url_str) && !all_links.contains(&link_url_str) {
                        to_visit.push_back(link_url_str.clone());
                        all_links.insert(link_url_str);
                    }
                }
            }
        }
    }

    // Output results
    println!("Total pages and folders analyzed: {}", visited.len());
    println!("All links saved to: {}", output_file);

    Ok(())
}
