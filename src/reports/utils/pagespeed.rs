use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::{self, Value};
use std::fs;
// Import for handling dynamic JSON
use std::io::{self, Write};
use std::path::Path;

use crate::reports::utils;

// const API_KEY: &str = "AIzaSyCCZu9Qxvkv8H0sCR9YPP7aP6CCQTZHFt8";

#[derive(Deserialize, Debug)]
struct LighthouseResult {
    categories: Categories,
    audits: Option<Audits>,
}

#[derive(Default, Deserialize, Debug)]
struct Categories {
    accessibility: Option<ScoreMetric>,
    best_practices: Option<ScoreMetric>,
    seo: Option<ScoreMetric>,
    pwa: Option<ScoreMetric>,
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
    other: std::collections::HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
struct Audit {
    id: String,
    title: String,
    description: Option<String>,
    #[serde(rename = "displayValue")]
    display_value: Option<String>,
    details: Option<AuditDetails>,
}

#[derive(Deserialize, Debug)]
struct Audits {
    #[serde(rename = "bootup-time")]
    bootup_time: Option<Audit>,
    #[serde(rename = "largest-contentful-paint")]
    largest_contentful_paint: Option<Audit>,
}

#[derive(Deserialize, Debug)]
struct PageSpeedResponse {
    #[serde(rename = "lighthouseResult")]
    lighthouse_result: LighthouseResult,
}

// Check if the API KEY ID exists in the .rustyfrog folder
fn api_check() -> Result<String, std::io::Error> {
    let dir_path = ".rustyfrog";
    let file_path = format!("{}/API_KEY.json", dir_path);

    if Path::new(&file_path).exists() {
        // If the API key file exists, read and return its content
        let api_key_from_file = fs::read_to_string(&file_path)?;
        Ok(api_key_from_file)
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
        let api_key = api_key.trim(); // Trim whitespace/newline

        // Write the API key to the file
        fs::write(&file_path, api_key)?;

        // read the api key from the file
        let api_key = fs::read_to_string(&file_path)?;

        // return the api key
        let API_KEY: String = api_key;

        println!("API key written to file: {}", API_KEY);

        Ok(API_KEY)
    }
}

pub async fn fetch_page_speed(url: &str) -> Result<(), ReqwestError> {
    let API_KEY = api_check().expect("Failed to read API_KEY");

    let message = format!("Fetching PageSpeed data for: {}", url);
    utils::loading::loading(message, 3);

    let client = reqwest::Client::new();
    let api_url = format!(
        "https://www.googleapis.com/pagespeedonline/v5/runPagespeed?key={}&url={}",
        API_KEY, url
    );

    let response = client.get(&api_url).send().await?;
    let body = response.text().await?;

    // Deserialize JSON into PageSpeedResponse struct
    let page_speed_response: PageSpeedResponse =
        serde_json::from_str(&body).expect("Failed to parse JSON");

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

        // Access and print LCP details
        if let Some(largest_contentful_paint) = &audits.largest_contentful_paint {
            println!("Largest Contentful Paint Audit:");
            println!("  ID: {}", largest_contentful_paint.id);
            println!("  Title: {}", largest_contentful_paint.title);
            if let Some(description) = &largest_contentful_paint.description {
                println!("  Description: {}", description);
            }
            if let Some(display_value) = &largest_contentful_paint.display_value {
                println!("  Display Value: {}", display_value);
            }
            if let Some(details) = &largest_contentful_paint.details {
                println!("  Details:");
                for heading in &details.headings {
                    println!("    Heading: {} - {}", heading.key, heading.label);
                }
                for item in &details.items {
                    println!("    Item: {:?}", item.other);
                }
            }
        }
    }

    // return the metrics that matter from the function

    Ok(())
}
