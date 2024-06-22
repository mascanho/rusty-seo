use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

// Function to create an HTML file with the provided HTML template content
pub fn create_html_file() -> Result<(), Box<dyn std::error::Error>> {
    let html_report = r#"

<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link rel="stylesheet" href="styles.css" />
  <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap" rel="stylesheet">
  <title>RustyFrog - SEO Report</title>
  <style>
    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }

    body {
      font-family: 'Roboto', sans-serif;
      background-color: #c4e17f; /* Screaming Frog green background */
      color: #333; /* Dark text color */
      margin: 0;
      padding: 0;
      display: flex;
      flex-direction: column;
      min-height: 100vh;
    }

    header {
      background: linear-gradient(135deg, #CE412B, #AD3425); /* Rust gradient for header */
      color: #FFFFFF; /* White text color */
      text-align: center;
      padding: 20px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      z-index: 1; /* Ensure header stays above content */
    }

    .logo img {
      width: 100px; /* Adjust as needed */
    }

    header span {
      color: #FFFFFF; /* White text color */
      display: block;
      margin-top: 5px;
      font-size: 0.9em;
    }

    header h1 {
      font-size: 1.8em;
      margin: 10px 0;
    }

    main {
      flex: 1;
      display: flex;
      flex-direction: column;
      align-items: center; /* Center sections horizontally */
      padding: 20px;
      margin: 2.5em 0;
    }

    .section {
      background-color: #FFFFFF; /* White background */
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
      width: 100%;
      max-width: 600px; /* Limit section width */
      margin-bottom: 20px;
      overflow: hidden; /* Ensure rounded corners clip shadow */
    }


.section ul {
  list-style-type: none; /* Remove default list styles */
  padding: 0;
}

.section ul li {
  margin-bottom: 8px;
  line-height: 1.4;
  display: flex; /* Use flexbox for alignment */
  align-items: center; /* Center items vertically */
}

.section ul li::before {
  content: "•"; /* Bullet point character */
  color: #CE412B; /* Match the section header color */
  font-size: 1.2em;
  margin-right: 0.5em; /* Adjust spacing between bullet and text */
  margin-left: 0.5em;
}

    .section h2 {
      background: linear-gradient(135deg, #CE412B, #AD3425); /* Rust gradient for section headers */
      color: #FFFFFF; /* White text color */
      font-size: 1.5em;
      padding: 15px;
      margin: 0;
      cursor: pointer;
      transition: background-color 0.3s ease;
      display: flex;
      justify-content: space-between;
      align-items: center;
      border-top-left-radius: 8px;
      border-top-right-radius: 8px;
      font-weight: 700; /* Bold font weight */
    }

    .section h2 span {
      font-size: 1.2em;
    }

    .section h2:hover {
      background: linear-gradient(135deg, #AD3425, #8B281F); /* Darker Rust gradient on hover */
    }

    .content {
      padding: 15px;
      display: none;
    }

    .content h4 {
    margin-top: 10px;
    color: gray;
    }

    .open > .content {
      display: block;
    }

    .section ul {
      list-style-type: none; /* Remove default list styles */
      padding: 0;
    }

    .section ul li {
      margin-bottom: 8px;
      line-height: 1.4;
    }

  pre, code {
      background: #f4f4f4; /* Light gray background */
      padding: 10px;
      border-radius: 5px;
      font-family: 'Courier New', Courier, monospace;
      overflow-x: auto;
      white-space: pre-wrap; /* CSS3 */
      white-space: -moz-pre-wrap; /* Firefox */
      white-space: -pre-wrap; /* Opera <7 */
      white-space: -o-pre-wrap; /* Opera 7 */
      word-wrap: break-word; /* IE */
      line-height: 1.9em; /* Set line height for code */
    }

    footer {
      background: linear-gradient(135deg, #AD3425, #8B281F); /* Adjusted Rust gradient for footer */
      color: #FFFFFF; /* White text color */
      text-align: center;
      padding: 20px;
      width: 100%;
      box-shadow: 0 -2px 4px rgba(0, 0, 0, 0.1); /* Drop shadow at the top */
      z-index: 1; /* Ensure footer stays above content */
      margin-top: auto; /* Push footer to bottom */
      font-size: 0.9em; /* Smaller font size */
    }

    footer a {
      color: #FFFFFF; /* White text color */
      text-decoration: none;
      font-weight: 700; /* Bold font weight */
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
<!-- Prism.js for syntax highlighting -->
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.24.1/themes/prism.min.css" />
  <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.24.1/prism.min.js"></script>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.24.1/components/prism-json.min.js"></script>
  <script>
    document.addEventListener("DOMContentLoaded", function() {
      var headers = document.querySelectorAll(".section h2");
      headers.forEach(function(header) {
        header.addEventListener("click", function() {
          var section = header.parentElement;
          section.classList.toggle("open");
          var icon = header.querySelector("span");
          if (section.classList.contains("open")) {
            icon.textContent = "🔽"; // Emoji for open
          } else {
            icon.textContent = "▶️"; // Emoji for closed
          }
        });
      });
    });
  </script>
</head>
<body>
  <header>
    <div class="logo">
      <img src="screaming-frog-logo.png" alt="Screaming Frog Logo" />
    </div>
    <h1>SEO Report for {{ seo_data.title | default(value="Untitled") }}</h1>
    <span>{{ seo_data.url }}</span>
  </header>

  <main>
    <div class="section">
      <h2>Meta Description <span>▶️</span></h2>
      <div class="content">
        <p>{{ seo_data.meta_description | default(value="No meta description found") }}</p>
      </div>
    </div>

    <div class="section">
      <h2>Flesch Score <span>▶️</span></h2>
      <div class="content">
        <h3>{{ seo_data.flesch_score }}</h3> 
        <h4> {{ seo_data.classification }} </h4>
      </div>
    </div>
 

 <div class="section">
      <h2>Headings <span>▶️</span></h2>
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
      <h2>Image Alt Texts <span>▶️</span></h2>
      <div class="content">
        <ul>
          {% for alt_text in seo_data.image_alt_texts %}
          <li>{{ alt_text }}</li>
          {% endfor %}
        </ul>
      </div>
    </div>

    <div class="section">
      <h2>Meta Keywords <span>▶️</span></h2>
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
      <h2>Relative Links <span>▶️</span></h2>
      <div class="content">
        <ul>
          {% for link in seo_data.internal_links %}
          <li><a href="{{ link }}">{{ link }}</a></li>
          {% endfor %}
        </ul>
      </div>
    </div>

    <div class="section">
      <h2>Absolute Links <span>▶️</span></h2>
      <div class="content">
        <ul>
          {% for link in seo_data.external_links %}
          <li><a href="{{ link }}">{{ link }}</a></li>
          {% endfor %}
        </ul>
      </div>
    </div>
 <div class="section">
      <h2>Structured Data <span>▶️</span></h2>
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
