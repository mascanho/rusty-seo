use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use url::ParseError;

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
    let url_input = ask_for_url()?;

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
