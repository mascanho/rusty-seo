use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use tera::{Context, Tera};

use crate::libs;

// Function to read user input for URL
fn user_input() -> Result<String, Box<dyn Error>> {
    println!("Please enter the URL of the website to analyze (e.g., https://example.com): ");
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input)?; // Read user input from stdin
    Ok(url_input.trim().to_string()) // Trim whitespace and return the URL
}

#[derive(Serialize)]
struct SEOData {
    url: Option<String>,
    title: Option<String>,
    meta_description: Option<String>,
    meta_keywords: Option<String>, // Change to Option<String> to handle optional meta keywords
    headings: HashMap<String, Vec<String>>,
    image_alt_texts: Vec<String>,
    internal_links: Vec<String>,
    external_links: Vec<String>,
    json_ld: serde_json::Value,
}

// Function to fetch HTML content from a URL
async fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new(); // Create a new reqwest client
    let response = client
        .get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?; // Send the HTTP GET request asynchronously

    if !response.status().is_success() {
        return Err(format!("Failed to fetch URL: {}", response.status()).into());
    }

    Ok(response.text().await?) // Return the HTML content as a string
}

// function to analyse flesch reading ease from website

// Function to analyze SEO metrics from HTML
fn analyze_seo(html: &str) -> SEOData {
    let document = Html::parse_document(html);

    // display the url being crawled
    let url = document
        .select(&Selector::parse("meta[property='og:url']").unwrap())
        .next()
        .and_then(|elem| elem.value().attr("content").map(String::from));

    // Extract title
    let title = document
        .select(&Selector::parse("title").unwrap())
        .next()
        .map(|elem| elem.text().collect::<String>());

    // Extract meta description
    let meta_description = document
        .select(&Selector::parse("meta[name='description']").unwrap())
        .next()
        .and_then(|elem| elem.value().attr("content").map(String::from));

    // Extract meta keywords
    let meta_keywords_selector = Selector::parse("meta[name='keywords']").unwrap();
    let meta_keywords = document
        .select(&meta_keywords_selector)
        .flat_map(|elem| elem.value().attr("content"))
        .next()
        .map(String::from);

    // Extract headings (h1-h6)
    let headings_selector = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
    let mut headings: HashMap<String, Vec<String>> = HashMap::new();

    for heading in document.select(&headings_selector) {
        let tag_name = heading.value().name().to_string();
        let heading_text = heading.text().collect::<String>();
        headings
            .entry(tag_name)
            .or_insert(Vec::new())
            .push(heading_text);
    }

    // Extract image alt texts
    let image_selector = Selector::parse("img").unwrap();
    let image_alt_texts: Vec<String> = document
        .select(&image_selector)
        .filter_map(|elem| elem.value().attr("alt").map(String::from))
        .collect();

    // Extract internal and external links
    let link_selector = Selector::parse("a").unwrap();
    let (internal_links, external_links): (Vec<String>, Vec<String>) = document
        .select(&link_selector)
        .map(|link| link.value().attr("href").unwrap_or("").to_string())
        .partition(|link| link.starts_with('/'));

    // exctract the structured data inside the json-ld and make it pretty and parse it in the html
    // template
    let json_ld = document
        .select(&Selector::parse("script[type='application/ld+json']").unwrap())
        .next()
        .and_then(|elem| {
            elem.text()
                .collect::<String>()
                .parse::<serde_json::Value>()
                .ok()
        })
        .unwrap_or(serde_json::Value::Null);

    // Extract the copy from the website and evaluate its flesch score

    let copy = document
        .select(&Selector::parse("p, h1, h2,h3,h4,h5,h6, span").unwrap())
        .map(|elem| elem.text().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");
    let flesch_score = flesch::flesch_reading_ease(&copy);

    // Initialize SEOData struct
    SEOData {
        json_ld,
        url,
        title,
        meta_description,
        meta_keywords,
        headings,
        image_alt_texts,
        internal_links,
        external_links,
    }
}

// Function to generate a full SEO report
pub async fn generate_full_report() -> Result<(), Box<dyn Error>> {
    // check for folder and files
    libs::create_html_file().unwrap();

    let url = user_input()?; // Read user input for URL
    let html = fetch_html(&url).await?; // Fetch HTML content from the provided URL
    let seo_data = analyze_seo(&html); // Analyze SEO metrics from the fetched HTML

    let tera = Tera::new("./rustyseo/**/*")?; // Initialize Tera template engine
    let mut context = Context::new();
    context.insert("seo_data", &seo_data); // Insert SEOData into Tera context

    let rendered = tera.render("report.html", &context)?; // Render HTML using Tera
    std::fs::write("seo_report.html", rendered)?; // Write rendered HTML to file

    println!("SEO report generated: seo_report.html");
    Ok(())
}
