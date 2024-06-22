use reqwest::Error as ReqwestError;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::reports::utils;

#[derive(Deserialize, Debug)]
struct LighthouseResult {
    categories: Categories,
    audits: Option<Audits>,
    performance_score: Option<f64>,
    dom_size: Option<ScoreMetric>,
}

#[derive(Default, Deserialize, Debug)]
struct Categories {
    accessibility: Option<ScoreMetric>,
    best_practices: Option<ScoreMetric>,
    seo: Option<ScoreMetric>,
    pwa: Option<ScoreMetric>,
    performance: Option<ScoreMetric>,
}

#[derive(Default, Deserialize, Debug)]
struct ScoreMetric {
    score: f64,
}

#[derive(Deserialize, Debug)]
struct AuditDetails {
    headings: Vec<AuditHeading>,
    items: Vec<AuditItem>,
}

#[derive(Deserialize, Debug)]
struct AuditHeading {
    key: String,
    label: String,
    #[serde(rename = "valueType")]
    value_type: String,
}

#[derive(Deserialize, Debug)]
struct AuditItem {
    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct Audit {
    id: String,
    title: String,
    description: Option<String>,
    #[serde(rename = "displayValue")]
    display_value: Option<String>,
    details: Option<AuditDetails>,
    score: Option<f64>,
    domains: Option<Vec<String>>,
    dom_size: Option<ScoreMetric>, // Corrected to match JSON structure
}

#[derive(Deserialize, Debug)]
struct Audits {
    #[serde(rename = "bootup-time")]
    bootup_time: Option<Audit>,
    #[serde(rename = "largest-contentful-paint")]
    largest_contentful_paint: Option<Audit>,
    #[serde(rename = "dom_size")] // Corrected field name
    dom_size: Option<Audit>, // Use Audit instead of AuditDetails if it's a single audit item
    score: Option<Audit>,
    interactive: Option<Audit>, // Assuming interactive audit has similar structure
}

#[derive(Deserialize, Debug)]
struct PageSpeedResponse {
    #[serde(rename = "lighthouseResult")]
    lighthouse_result: LighthouseResult,
}

// Check if the API KEY ID exists in the .rustyfrog folder
fn api_check() -> Result<String, io::Error> {
    let dir_path = ".rustyfrog";
    let file_path = format!("{}/API_KEY.json", dir_path);

    if Path::new(&file_path).exists() {
        // If the API key file exists, read and return its content
        let api_key_from_file = fs::read_to_string(&file_path)?;
        Ok(api_key_from_file.trim().to_string())
    } else {
        // Create the folder if it doesn't exist
        if !Path::new(dir_path).exists() {
            fs::create_dir(dir_path)?;
        }

        // Prompt the user for the API key
        print!("Please enter your API key: ");
        io::stdout().flush()?; // Make sure the prompt is shown before reading input
        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key)?;
        let api_key = api_key.trim().to_string(); // Trim whitespace/newline

        // Write the API key to the file
        fs::write(&file_path, &api_key)?;

        println!("API key written to file: {}", api_key);

        Ok(api_key)
    }
}

pub async fn fetch_page_speed(url: &str) -> Result<(), ReqwestError> {
    let api_key = api_check().expect("Failed to read API_KEY");

    let message = format!("Fetching PageSpeed data for: {}", url);
    utils::loading::loading(message, 3);

    let client = reqwest::Client::new();
    let api_url = format!(
        "https://www.googleapis.com/pagespeedonline/v5/runPagespeed?key={}&url={}",
        api_key, url
    );

    let response = client.get(&api_url).send().await?;
    let body = response.text().await?;

    // Deserialize JSON into PageSpeedResponse struct
    let page_speed_response: PageSpeedResponse = serde_json::from_str(&body).unwrap();

    // Access and print bootup time details
    if let Some(audits) = &page_speed_response.lighthouse_result.audits {
        if let Some(bootup_time) = &audits.bootup_time {
            println!("Bootup Time Audit:");
            println!("  ID: {}", bootup_time.id);
            println!("  Title: {}", bootup_time.title);
            if let Some(description) = &bootup_time.description {
                println!("  Description: {}", description);
            }
            if let Some(display_value) = &bootup_time.display_value {
                println!("  Display Value: {}", display_value);
            }
            if let Some(details) = &bootup_time.details {
                println!("  Details:");
                for heading in &details.headings {
                    println!("    Heading: {} - {}", heading.key, heading.label);
                }
                for item in &details.items {
                    println!("    Item: {:?}", item.other);
                }
            }
        }

        // Access and print Largest Contentful Paint details
        if let Some(interactive) = &audits.interactive {
            println!("Interactive Audit:");
            println!("  ID: {}", interactive.id);
            println!("  Title: {}", interactive.title);
            if let Some(description) = &interactive.description {
                println!("  Description: {}", description);
            }
            if let Some(display_value) = &interactive.display_value {
                println!("  Display Value: {}", display_value);
            }
            if let Some(score) = &interactive.score {
                println!("  Score: {}", score);
            }
        }

        // Access and print dom-size details
        if let Some(dom_size) = &audits.dom_size {
            println!("DOM Size Audit:");
            if let Some(dom_size_score) = &dom_size.score {
                println!("  Score: {}", dom_size_score);
            } else {
                println!("  Score not available");
            }
        }
    }

    Ok(())
}
