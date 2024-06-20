use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::Serialize;
use std::error::Error;
use std::{collections::HashMap, io};
use tera::{Context, Tera};
use tokio;

// Function to read user input for URL
fn user_input() -> Result<String, Box<dyn Error>> {
    println!("Please enter the URL of the website to analyze (e.g., https://example.com): ");
    let mut url_input = String::new();

    io::stdin().read_line(&mut url_input)?; // Read user input from stdin

    Ok(url_input.trim().to_string()) // Trim whitespace and return the URL
}

#[derive(Serialize)]
struct SEOData {
    title: Option<String>,
    meta_description: Option<String>,
    meta_keywords: Option<String>,
    headings: HashMap<String, Vec<String>>,
    image_alt_texts: Vec<String>,
    keyword_density: f64,
    link_density: f64,
    internal_links: Vec<String>,
    external_links: Vec<String>,
    external_link_density: f64,
    unique_internal_links: f64,
    unique_external_links: f64,
    internal_link_density: f64,
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

fn analyze_seo(html: &str) -> SEOData {
    let document = Html::parse_document(html);

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
    let meta_keywords = document
        .select(&Selector::parse("meta[name='keywords']").unwrap())
        .next()
        .and_then(|elem| elem.value().attr("content").map(String::from));

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
    let mut image_alt_texts = Vec::new();

    for img in document.select(&image_selector) {
        if let Some(alt) = img.value().attr("alt") {
            image_alt_texts.push(alt.to_string());
        }
    }

    // Create SEOData struct
    SEOData {
        title,
        meta_description,
        meta_keywords,
        headings,
        image_alt_texts,
        keyword_density: 0.0, // Placeholder values
        link_density: 0.0,
        internal_links: Vec::new(),
        external_links: Vec::new(),
        external_link_density: 0.0,
        unique_internal_links: 0.0,
        unique_external_links: 0.0,
        internal_link_density: 0.0,
    }
}

pub async fn generate_full_report() -> Result<(), Box<dyn Error>> {
    let html = fetch_html(&user_input()?).await?;
    let seo_data = analyze_seo(&html);

    let tera = Tera::new("templates/**/*")?;
    let mut context = Context::new();
    context.insert("seo_data", &seo_data);

    let rendered = tera.render("report.html", &context)?;
    std::fs::write("seo_report.html", rendered)?;

    println!("SEO report generated: seo_report.html");
    Ok(())
}
