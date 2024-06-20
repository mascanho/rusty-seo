use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

// Function to create an HTML file with the provided HTML template content
pub fn create_html_file() -> Result<(), Box<dyn std::error::Error>> {
    let html_report = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="styles.css" />
    <title>SEO Report</title>
  </head>
  <body>
    <h1>SEO Report for {{ seo_data.title | default(value="Untitled") }}</h1>

    <div class="section">
      <h2>Meta Description</h2>
      <p>
        {{ seo_data.meta_description | default(value="No meta description found") }}
      </p>
    </div>

    <div class="section">
      <h2>Headings</h2>
      {% for tag, texts in seo_data.headings %}
      <h3>{{ tag | upper }}</h3>
      <ul>
        {% for text in texts %}
        <li>{{ text }}</li>
        {% endfor %}
      </ul>
      {% endfor %}
    </div>

    <div class="section">
      <h2>Image Alt Texts</h2>
      <ul>
        {% for alt_text in seo_data.image_alt_texts %}
        <li>{{ alt_text }}</li>
        {% endfor %}
      </ul>
    </div>

    <div class="section">
      <h2>Meta Keywords</h2>
      {% if seo_data.meta_keywords %}
      <ul>
        {% for kw in seo_data.meta_keywords %}
        <li>{{ kw }}</li>
        {% endfor %}
      </ul>
      {% else %}
      <p>No meta keywords found</p>
      {% endif %}
    </div>

    <div class="section">
      <h2>Internal Links</h2>
      <ul>
        {% for link in seo_data.internal_links %}
        <li><a href="{{ link }}">{{ link }}</a></li>
        {% endfor %}
      </ul>
    </div>

    <div class="section">
      <h2>External Links</h2>
      <ul>
        {% for link in seo_data.external_links %}
        <li><a href="{{ link }}">{{ link }}</a></li>
        {% endfor %}
      </ul>
    </div>
  </body>
</html>"#;

    // Specify the path for the HTML file
    let file_path = ".report.html";

    // Create or open the file for writing
    let mut file = File::create(file_path)?;

    // Write the HTML content to the file
    file.write_all(html_report.as_bytes())?;

    println!("HTML file '{}' created successfully.", file_path);

    // Convert the file to UTF-8
    convert_to_utf8(file_path)?;

    Ok(())
}

// Function to convert a file to UTF-8 encoding
fn convert_to_utf8(file_path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = file_path.as_ref();

    // Read the file content
    let content = fs::read_to_string(file_path)?;

    // Write back to the file with UTF-8 encoding
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    println!(
        "Converted file '{}' to UTF-8 successfully.",
        file_path.display()
    );

    Ok(())
}
