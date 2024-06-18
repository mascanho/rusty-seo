use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use url::Url;

mod crawlers;

fn crawl_all() -> Result<(), Box<dyn Error>> {
    let mut url_input = String::new();
    io::stdin().read_line(&mut url_input)?;
    let url_input = url_input.trim();

    // Prompt the user for a URL
    print!("Please enter the URL of the website to analyze (e.g., https://example.com): ");
    io::stdout().flush()?;

    // Prompt the user for parameters to ignore
    print!("Enter URL parameters to ignore, separated by commas (e.g., utm_source,session_id): ");
    io::stdout().flush()?;

    let mut ignore_params_input = String::new();
    io::stdin().read_line(&mut ignore_params_input)?;
    let ignore_params_input = ignore_params_input.trim();

    // Parse the list of parameters to ignore
    let ignore_params: HashSet<String> = ignore_params_input
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    println!("Ignoring the following URL parameters: {:?}", ignore_params);

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
        visited.insert(current_url.clone());

        // Fetch the HTML content of the current page
        let response = match client.get(&current_url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send() {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("Error fetching {}: {}", current_url, err);
                    continue;
                },
            };

        let response_text = match response.text() {
            Ok(text) => text,
            Err(err) => {
                eprintln!("Error reading response text for {}: {}", current_url, err);
                continue;
            }
        };

        // Parse the HTML
        let document = Html::parse_document(&response_text);

        // Select all anchor tags
        let link_selector = Selector::parse("a").unwrap();
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                match base_url.join(href) {
                    Ok(link_url) => {
                        if link_url.domain() == Some(base_domain.as_str()) {
                            // Check if URL contains any ignored parameters or invalid characters
                            let contains_ignored_param = link_url
                                .query_pairs()
                                .any(|(key, _)| ignore_params.contains(&key.to_string()));

                            let contains_invalid_chars =
                                link_url.as_str().contains('#') || link_url.as_str().contains('?');

                            if contains_ignored_param || contains_invalid_chars {
                                println!("Skipping URL with ignored parameters or invalid characters: {}", link_url);
                                continue;
                            }

                            let link_url_str = link_url.to_string();
                            if !visited.contains(&link_url_str)
                                && !all_links.contains(&link_url_str)
                            {
                                to_visit.push_back(link_url_str.clone());
                                all_links.insert(link_url_str.clone());
                                writeln!(file_writer, "{}", link_url_str)?; // Write to file here
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
    }
    // Output results
    println!("Total pages and folders analyzed: {}", visited.len());
    println!("All links saved to: {}", output_file);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Print asci art
    println!(
        r#"

|  \     /  \                                      |  \               /      \                                  |  \                    
| $$\   /  $$  ______    ______    _______   ______| $$_______       |  $$$$$$\  ______   ______   __   __   __ | $$  ______    ______  
| $$$\ /  $$$ |      \  /      \  /       \ /      \\$/       \      | $$   \$$ /      \ |      \ |  \ |  \ |  \| $$ /      \  /      \ 
| $$$$\  $$$$  \$$$$$$\|  $$$$$$\|  $$$$$$$|  $$$$$$\|  $$$$$$$      | $$      |  $$$$$$\ \$$$$$$\| $$ | $$ | $$| $$|  $$$$$$\|  $$$$$$\
| $$\$$ $$ $$ /      $$| $$   \$$| $$      | $$  | $$ \$$    \       | $$   __ | $$   \$$/      $$| $$ | $$ | $$| $$| $$    $$| $$   \$$
| $$ \$$$| $$|  $$$$$$$| $$      | $$_____ | $$__/ $$ _\$$$$$$\      | $$__/  \| $$     |  $$$$$$$| $$_/ $$_/ $$| $$| $$$$$$$$| $$      
| $$  \$ | $$ \$$    $$| $$       \$$     \ \$$    $$|       $$       \$$    $$| $$      \$$    $$ \$$   $$   $$| $$ \$$     \| $$      
 \$$      \$$  \$$$$$$$ \$$        \$$$$$$$  \$$$$$$  \$$$$$$$         \$$$$$$  \$$       \$$$$$$$  \$$$$$\$$$$  \$$  \$$$$$$$ \$$      
    "#
    );

    // Prompt the user for the options
    println!("Please select an option:");
    println!("1. Crawl the entire website");
    println!("2. Crawl a specific URL");
    println!("3. Crawl a page and get its Headings");
    println!("4. Crawl a website and generate a sitemap");

    // Read the user's choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    // Parse the user's choice
    let choice = match choice.trim().parse::<i32>() {
        Ok(choice) => choice,
        Err(_) => {
            println!("Invalid choice. Exiting...");
            return Ok(());
        }
    };

    match choice {
        1 => crawl_all()?,
        2 => {
            // Crawl a specific URL
            crawlers::crawl_page()?;
        }
        3 => {
            // Crawl a page and get its headings
            crawlers::get_headings().expect("Failed to get headings");
        }
        4 => {
            // Crawl a website and generate a sitemap
            crawlers::generate_sitemaps().expect("Failed to generate sitemap");
        }
        _ => {
            println!("Invalid choice. Exiting...");
            return Ok(());
        }
    }

    Ok(())
}
