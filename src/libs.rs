use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

// Function to create an HTML file with the provided HTML template content
pub fn create_html_file() -> Result<(), Box<dyn std::error::Error>> {
    let html_report = r#"

<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=San+Francisco:wght@400;600&display=swap">
  <title>RustyFrog - SEO Report</title>
  <style>
    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }

    body {
      font-family: 'San Francisco', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
      background-color: #F5F5F7; /* Light gray background */
      color: #1D1D1F; /* Dark text color */
      margin: 0;
      padding: 0;
      display: flex;
      flex-direction: column;
      min-height: 100vh;
    }

    header {
      background: #FFFFFF; /* White background */
      color: #1D1D1F; /* Dark text color */
      text-align: center;
      padding: 20px;
      box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
      z-index: 1;
    }

    .logo img {
      width: full; /* Adjust as needed */
      height: 100px;
      object-fit: cover;
    }

    header h1 {
      font-size: 2em;
      margin: 10px 0;
      font-weight: 600;
    }

    header span {
      color: #6e6e73; /* Light gray text color */
      display: block;
      margin-top: 5px;
      font-size: 1em;
    }

    main {
      flex: 1;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 20px;
      margin: 2.5em 0;
    }

    .section {
      background-color: #FFFFFF; /* White background */
      border-radius: 10px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      width: 100%;
      max-width: 800px; /* Limit section width */
      margin-bottom: 20px;
      overflow: hidden;
    }

.section ul {
      list-style-type: none; /* Remove default list styles */
      padding-left: 10px; /* Add left padding for custom bullets */
      margin: 0;
    }

.section ul li {
  position: relative; /* Position relative for pseudo-element */
  margin-bottom: 8px;
  line-height: 1.6;
  padding-left: 20px; /* Space for bullet */
}

.section ul li::before {
  content: ""; /* Pseudo-element for custom bullet */
  position: absolute;
  left: 0;
  top: 50%; /* Center vertically */
  transform: translateY(-50%); /* Correct centering */
  width: 8px;
  height: 8px;
  background-color: #0070C9; /* Apple blue color */
  border-radius: 50%; /* Circle shape */
}
    .section h2 {
      background: #F5F5F7; /* Light gray background */
      color: #1D1D1F; /* Dark text color */
      font-size: 1.5em;
      padding: 15px;
      margin: 0;
      cursor: pointer;
      transition: background-color 0.3s ease;
      display: flex;
      justify-content: space-between;
      align-items: center;
      border-top-left-radius: 10px;
      border-top-right-radius: 10px;
      font-weight: 600; /* Bold font weight */
    }

    .section h2:hover {
      background: #e5e5e7; /* Slightly darker gray on hover */
    }

    .content {
      padding: 15px;
      display: none;
    }

    .content h3 {
      margin: 10px 0;
      color: #1D1D1F; /* Dark text color */
    }

    .content h4 {
      margin: 10px 0;
      color: #6e6e73; /* Light gray text color */
    }

    .open > .content {
      display: block;
    }

    pre, code {
      background: #F5F5F7; /* Light gray background */
      padding: 10px;
      border-radius: 5px;
      font-family: 'Courier New', Courier, monospace;
      overflow-x: auto;
      white-space: pre-wrap;
      word-wrap: break-word;
      line-height: 1.6em; /* Set line height for code */
    }

    footer {
      background: #FFFFFF; /* White background */
      color: #6e6e73; /* Light gray text color */
      text-align: center;
      padding: 20px;
      width: 100%;
      box-shadow: 0 -1px 3px rgba(0, 0, 0, 0.1); /* Drop shadow at the top */
      z-index: 1;
      margin-top: auto; /* Push footer to bottom */
      font-size: 1em; /* Regular font size */
    }

    footer a {
      color: #0070C9; /* Apple blue link color */
      text-decoration: none;
      font-weight: 600; /* Bold font weight */
    }

    .footer-content {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .footer-content p {
      margin: 5px 0;
    }

    .heart {
      color: #ff4136; /* Red heart color */
    }
  </style>
  <script>
    document.addEventListener("DOMContentLoaded", function() {
      var headers = document.querySelectorAll(".section h2");
      headers.forEach(function(header) {
        header.addEventListener("click", function() {
          var section = header.parentElement;
          section.classList.toggle("open");
          var icon = header.querySelector("span");
          if (section.classList.contains("open")) {
            icon.textContent = "▼"; // Down arrow for open
          } else {
            icon.textContent = "▶"; // Right arrow for closed
          }
        });
      });
    });
  </script>
</head>
<body>
  <header>
    <div class="logo">
      <img src="https://miro.medium.com/v2/resize:fit:1024/1*k2vgFiXvADxh8woVOmPkSQ.png" alt="Screaming Frog Logo" />
    </div>
    <h1>SEO Report for {{ seo_data.title | default(value="Untitled") }}</h1>
    <span>{{ seo_data.url }}</span>
  </header>

  <main>
    <div class="section">
      <h2>Meta Description <span>▶</span></h2>
      <div class="content">
        <p>{{ seo_data.meta_description | default(value="No meta description found") }}</p>
      </div>
    </div>

    <div class="section">
      <h2>Flesch Score <span>▶</span></h2>
      <div class="content">
        <h3>{{ seo_data.flesch_score }}</h3> 
        <h4>{{ seo_data.classification }}</h4>
      </div>
    </div>

    <div class="section">
      <h2>Headings <span>▶</span></h2>
      <div class="content">
        {% for tag, texts in seo_data.headings %}
        <h3>{{ tag | upper }}</h3>
        <ul>
          {% for text in texts %}
          <li>{{ text }}</li>
          {% endfor %}
        </ul>
        {% endfor %}
      </div>
    </div>

    <div class="section">
      <h2>Image Alt Texts <span>▶</span></h2>
      <div class="content">
        <ul>
          {% for alt_text in seo_data.image_alt_texts %}
          <li>{{ alt_text }}</li>
          {% endfor %}
        </ul>
      </div>
    </div>

    <div class="section">
      <h2>Meta Keywords <span>▶</span></h2>
      <div class="content">
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
    </div>

    <div class="section">
      <h2>Relative Links <span>▶</span></h2>
      <div class="content">
        <ul>
          {% for link in seo_data.internal_links %}
          <li>
            <a href="{{ link.href }}" 
               rel="{{ link.rel }}" 
               target="{{ link.target }}">{{ link.href }}</a>
          </li>
          {% endfor %}
        </ul>
      </div>
    </div>

    <div class="section">
      <h2>Absolute Links <span>▶</span></h2>
      <div class="content">
        <ul>
          {% for link in seo_data.external_links %}
          <li>
            <a href="{{ link.href }}" 
               rel="{{ link.rel }}" 
               target="{{ link.target }}">{{ link.href }}</a>
          </li>
          {% endfor %}
        </ul>
      </div>
    </div>

    <div class="section">
      <h2>Structured Data <span>▶</span></h2>
      <div class="content">
        <pre><code class="language-json">{{ seo_data.json_ld | json_encode(indent=4) | safe }}</code></pre>
      </div>
    </div>
  </main>

  <footer>
    <div class="footer-content">
      <p>&copy; 2024 All rights reserved.</p>
      <p>Designed with <span class="heart">&hearts;</span> by <a href="https://markwarrior.dev">Mark Warrior</a></p>
    </div>
  </footer>
</body>
</html>

"#;

    // Check if exists and create directory .rustyseo
    if !Path::new("./rustyseo").exists() {
        fs::create_dir("./rustyseo")?;
    }

    // Specify the path for the HTML file
    let file_path = "./rustyseo/report.html";

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
