use chrono::Utc;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use reqwest::Url;
use scraper::{Html, Selector};
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use url::ParseError;

// Prompt the user for URL
pub fn user_input() -> Result<String, Box<dyn Error>> {
    println!("Please enter the URL of the website to analyze (e.g., https://example.com): ");
    io::stdout().flush()?; // Ensure the prompt is displayed immediately

    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input)?;

    // Trim any extra whitespace or newline characters
    let url = url_input.trim().to_string();

    // Validate URL format
    Url::parse(&url)?;

    Ok(url)
}

pub fn structured_data() -> Result<(), Box<dyn Error>> {
    // Get user input for URL
    let url = user_input()?;

    // Fetch HTML content
    let client = Client::new();
    let response = client
        .get(&url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL: {}", response.status()).into());
    }

    let html = response.text()?;
    let document = Html::parse_document(&html);

    // Check for structured data (json-ld)
    let json_ld_selector = Selector::parse("script[type='application/ld+json']")
        .map_err(|e| format!("Failed to parse selector: {:?}", e))?;

    if let Some(script) = document.select(&json_ld_selector).next() {
        let json_ld_content = script.inner_html();
        // Parse the JSON content
        match serde_json::from_str::<Value>(&json_ld_content) {
            Ok(parsed_json) => {
                // Print the parsed JSON in a pretty, human-readable format
                println!("{}", serde_json::to_string_pretty(&parsed_json)?);
            }
            Err(e) => {
                return Err(format!("Failed to parse JSON-LD script: {:?}", e).into());
            }
        }
    } else {
        println!("No structured data found in JSON-LD script");
    }

    Ok(())
}

pub fn content_quality() -> Result<(), Box<dyn Error>> {
    // Get user input for URL
    let url = user_input()?;

    // Fetch HTML content
    let client = Client::new();
    let response = client.get(&url).send()?;
    let html = response.text()?;

    // Parse HTML
    let document = Html::parse_document(&html);

    // Extract text content from paragraphs
    let paragraph_selector = Selector::parse("p").unwrap();
    let mut content = String::new();
    for paragraph in document.select(&paragraph_selector) {
        content.push_str(paragraph.text().collect::<String>().as_str());
        content.push_str("\n");
    }

    // Perform basic readability analysis (Flesch-Kincaid readability score)
    let num_words = content.split_whitespace().count();
    let num_sentences = content.split_terminator('.').count()
        + content.split_terminator('!').count()
        + content.split_terminator('?').count();
    let num_syllables = content
        .split_whitespace()
        .map(|word| syllables_count(word))
        .sum::<usize>();

    let flesch_reading_ease = 206.835
        - 1.015 * (num_words as f64 / num_sentences as f64)
        - 84.6 * (num_syllables as f64 / num_words as f64);

    println!("Flesch Reading Ease Score: {:.2}", flesch_reading_ease);

    Ok(())
}

// Simple syllable counter (approximation)
fn syllables_count(word: &str) -> usize {
    word.chars().filter(|&c| "aeiouAEIOU".contains(c)).count()
}

pub fn get_headings() -> Result<Vec<(String, String)>, Box<dyn Error>> {
    // Prompt the user for URL
    print!("Please enter the URL of the website to analyze (e.g., https://example.com): ");
    io::stdout().flush()?;

    // Read user input
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    let url = url.trim();

    // Create a new reqwest client
    let client = Client::new();

    // Send a GET request to the URL
    let res = client.get(url).send()?;
    let body = res.text()?;

    // Parse the HTML body
    let document = Html::parse_document(&body);

    // Create selectors for all heading tags
    let heading_tags = vec!["h1", "h2", "h3", "h4", "h5", "h6"];
    let selectors: Vec<Selector> = heading_tags
        .iter()
        .map(|tag| Selector::parse(tag).unwrap())
        .collect();

    // Extract the text content from the heading tags along with the tag name
    let mut headings = Vec::new();
    for (tag, selector) in heading_tags.iter().zip(selectors.iter()) {
        for element in document.select(selector) {
            let heading_text = element.text().collect::<Vec<_>>().join(" ");
            headings.push((tag.to_string(), heading_text));
        }
    }

    // Print and return the headings with their tags
    for (tag, heading) in &headings {
        println!("{}: {}", tag.to_uppercase(), heading);
    }

    Ok(headings)
}

pub fn crawl_page() -> Result<(), Box<dyn Error>> {
    let mut page_url = String::new();
    println!("Please enter the URL of the website to analyze (e.g., https://example.com): ");
    io::stdin()
        .read_line(&mut page_url)
        .expect("Failed to read line");
    let _page_url = page_url.trim();

    Ok(())
}

pub fn generate_sitemaps() -> Result<(), Box<dyn Error>> {
    // Ask the user for the website URL
    let url_input = ask_for_url().expect("It needs user input...");

    // Ensure the URL ends with a slash for proper crawling logic
    let url_input = if url_input.ends_with('/') {
        url_input.clone()
    } else {
        format!("{}/", url_input)
    };

    // Parse the base URL to extract the domain
    let base_url = Url::parse(&url_input)?;

    // Ask the user for the output filename
    println!("Please enter the output filename: ");
    let mut filename_input = String::new();
    io::stdin()
        .read_line(&mut filename_input)
        .expect("Failed to read line");
    let filename_input = filename_input.trim();

    // Ensure the file that the user typed has extension .xml
    let filename = if filename_input.ends_with(".xml") {
        filename_input.to_string()
    } else {
        format!("{}.xml", filename_input)
    };

    // Create a new reqwest client
    let client = Client::new();

    // Create a file writer
    let file = File::create(&filename)?;
    let mut file_writer = BufWriter::new(file);

    // Output the sitemap header
    writeln!(
        &mut file_writer,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"
    )?;

    // Start crawling from the initial URL
    let mut visited_urls = vec![url_input.clone()];
    crawl_url(
        &url_input,
        &client,
        &base_url,
        &mut file_writer,
        &mut visited_urls,
    )?;

    // Output the sitemap footer
    writeln!(&mut file_writer, "</urlset>")?;

    // Show the URLs being created to the user on the terminal
    println!("Creating sitemap for: {}", url_input);

    Ok(())
}

// Function to recursively crawl URLs within the same domain
fn crawl_url(
    url: &str,
    client: &Client,
    base_url: &Url,
    mut file_writer: &mut BufWriter<File>,
    visited_urls: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let res = client.get(url).send()?;
    let body = res.text()?;
    let document = Html::parse_document(&body);

    // Extract links from anchor tags and link tags
    let selector = Selector::parse("a[href], link[href]").unwrap();
    for node in document.select(&selector) {
        if let Some(href) = node.value().attr("href") {
            match normalize_url(base_url, href) {
                Ok(normalized_url) => {
                    // Handle normalized URL
                    if is_same_domain(base_url, &normalized_url)
                        && !visited_urls.contains(&normalized_url)
                    {
                        visited_urls.push(normalized_url.clone());

                        // Write URL entry to the sitemap file
                        writeln!(&mut file_writer, "<url>")?;
                        writeln!(&mut file_writer, "<loc>{}</loc>", normalized_url)?;
                        writeln!(
                            &mut file_writer,
                            "<lastmod>{}</lastmod>",
                            Utc::now().to_rfc3339()
                        )?;
                        writeln!(&mut file_writer, "<changefreq>monthly</changefreq>")?;
                        writeln!(&mut file_writer, "<priority>0.5</priority>")?;
                        writeln!(&mut file_writer, "</url>")?;

                        println!("Found URL: {}", normalized_url);

                        // Recursively crawl pages within the same domain
                        crawl_url(
                            &normalized_url,
                            client,
                            base_url,
                            &mut file_writer,
                            visited_urls,
                        )?;
                    }
                }
                Err(err) => {
                    // Log or handle the error appropriately
                    eprintln!("Failed to normalize URL {}: {}", href, err);
                }
            }
        }
    }

    Ok(())
}

// Helper function to normalize URLs
fn normalize_url(base_url: &Url, href: &str) -> Result<String, ParseError> {
    // Attempt to resolve the href against the base URL
    let mut url = base_url.join(href)?;

    // Normalize the URL to handle special characters and percent encoding
    url.set_fragment(None);
    url.set_query(None);

    Ok(url.to_string())
}

// Helper function to check if a URL is within the same domain
fn is_same_domain(base_url: &Url, url: &str) -> bool {
    match Url::parse(url) {
        Ok(parsed_url) => parsed_url.domain() == base_url.domain(),
        Err(_) => false,
    }
}

// Helper function to ask for the website URL
fn ask_for_url() -> Result<String, io::Error> {
    println!("Please enter the URL of the website to analyze (e.g., https://example.com): ");
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input)?;
    Ok(url_input.trim().to_string())
}
