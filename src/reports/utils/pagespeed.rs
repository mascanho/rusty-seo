use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::{self, Value}; // Import for handling dynamic JSON

const API_KEY: &str = "AIzaSyCCZu9Qxvkv8H0sCR9YPP7aP6CCQTZHFt8";

#[derive(Deserialize, Debug)]
struct LighthouseResult {
    categories: Categories,
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
struct PageSpeedResponse {
    lighthouseResult: LighthouseResult,
}

pub async fn fetch_page_speed(url: &str) -> Result<(), ReqwestError> {
    let client = reqwest::Client::new();
    let api_url = format!(
        "https://www.googleapis.com/pagespeedonline/v5/runPagespeed?key={}&url={}",
        API_KEY, url
    );

    let response = client.get(&api_url).send().await?;
    let body = response.text().await?;

    let json: Value = serde_json::from_str(&body).expect("Failed to parse JSON response");
    let pretty_json = serde_json::to_string_pretty(&json).expect("Failed to format JSON response");

    println!("Formatted JSON Response:\n{}", pretty_json);

    Ok(())
}
