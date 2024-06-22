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
    performance: Option<ScoreMetric>,
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
    numeric_value: Option<f64>,    // Added for numeric value if present
    seo: Option<AuditItem>,
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
    #[serde(rename = "total-blocking-time")]
    total_blocking_time: Option<Audit>,
    #[serde(rename = "speed-index")]
    speed_index: Option<Audit>,
    performance: Option<Audit>,
    seo: Option<Audit>,
    redirects: Option<Audit>,
}

#[derive(Deserialize, Debug)]
struct PageSpeedResponse {
    #[serde(rename = "lighthouseResult")]
    lighthouse_result: LighthouseResult,
}

#[derive(Debug)]
pub struct AuditInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub display_value: Option<String>,
    pub score: Option<f64>,
    pub numeric_value: Option<f64>,
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

pub async fn fetch_page_speed(url: &str) -> Result<Vec<AuditInfo>, ReqwestError> {
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
    let mut audits_info: Vec<AuditInfo> = Vec::new();

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

        // Access and print Time To Interactive details
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

        // Access and print Largest Contentful Paint details
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
            if let Some(score) = &largest_contentful_paint.score {
                println!("  Score: {}", score);
            }
        }

        // Access and print dom-size details
        if let Some(dom_size) = &audits.dom_size {
            println!("DOM Size Audit:");
            println!("  ID: {}", dom_size.id);
            println!("  Title: {}", dom_size.title);
            if let Some(description) = &dom_size.description {
                println!("  Description: {}", description);
            }
            if let Some(display_value) = &dom_size.display_value {
                println!("  Display Value: {}", display_value);
            }
            if let Some(score) = &dom_size.numeric_value {
                println!("  Score: {}", score);
            }
        }

        // Access Total Blocking Time
        if let Some(total_blocking_time) = &audits.total_blocking_time {
            println!("Total Blocking Time Audit:");
            println!("  ID: {}", total_blocking_time.id);
            println!("  Title: {}", total_blocking_time.title);
            if let Some(description) = &total_blocking_time.description {
                println!("  Description: {}", description);
            }
            if let Some(display_value) = &total_blocking_time.display_value {
                println!("  Display Value: {}", display_value);
            }
            if let Some(score) = &total_blocking_time.score {
                println!("  Score: {}", score);
            }
        }
        // Access speed Index
        if let Some(speed_index) = &audits.speed_index {
            println!("Speed Index Audit:");
            println!("  ID: {}", speed_index.id);
            println!("  Title: {}", speed_index.title);
            if let Some(description) = &speed_index.description {
                println!("  Description: {}", description);
            }
            if let Some(display_value) = &speed_index.display_value {
                println!("  Display Value: {}", display_value);
            }
            if let Some(score) = &speed_index.score {
                println!("  Score: {}", score);
            }
        }
        // Get The redirects
        if let Some(redirects) = &audits.redirects {
            println!("Redirects Audit:");
            println!("  ID: {}", redirects.id);
            println!("  Title: {}", redirects.title);
            if let Some(description) = &redirects.description {
                println!("  Description: {}", description);
            }
            if let Some(display_value) = &redirects.display_value {
                println!("  Display Value: {}", display_value);
            }
            if let Some(domains) = &redirects.domains {
                println!("  Domains: {:?}", domains);
            }
            if let Some(details) = &redirects.details {
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

    // Push all the audit details into the audit_info vector
    // Access and collect audit information
    if let Some(audits) = &page_speed_response.lighthouse_result.audits {
        if let Some(bootup_time) = &audits.bootup_time {
            audits_info.push(AuditInfo {
                id: bootup_time.id.clone(),
                title: bootup_time.title.clone(),
                description: bootup_time.description.clone(),
                display_value: bootup_time.display_value.clone(),
                score: None,
                numeric_value: None,
            });
        }

        if let Some(interactive) = &audits.interactive {
            audits_info.push(AuditInfo {
                id: interactive.id.clone(),
                title: interactive.title.clone(),
                description: interactive.description.clone(),
                display_value: interactive.display_value.clone(),
                score: interactive.score,
                numeric_value: None,
            });
        }

        if let Some(largest_contentful_paint) = &audits.largest_contentful_paint {
            audits_info.push(AuditInfo {
                id: largest_contentful_paint.id.clone(),
                title: largest_contentful_paint.title.clone(),
                description: largest_contentful_paint.description.clone(),
                display_value: largest_contentful_paint.display_value.clone(),
                score: largest_contentful_paint.score,
                numeric_value: None,
            });
        }

        if let Some(dom_size) = &audits.dom_size {
            audits_info.push(AuditInfo {
                id: dom_size.id.clone(),
                title: dom_size.title.clone(),
                description: dom_size.description.clone(),
                display_value: dom_size.display_value.clone(),
                score: None,
                numeric_value: dom_size.numeric_value,
            });
        }

        if let Some(total_blocking_time) = &audits.total_blocking_time {
            audits_info.push(AuditInfo {
                id: total_blocking_time.id.clone(),
                title: total_blocking_time.title.clone(),
                description: total_blocking_time.description.clone(),
                display_value: total_blocking_time.display_value.clone(),
                score: total_blocking_time.score,
                numeric_value: None,
            });
        }
    }

    println!("auditInfoL {:#?}", audits_info);

    Ok(audits_info)
}
