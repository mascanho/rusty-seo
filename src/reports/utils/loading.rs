use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

pub fn loading(text: String, duration_secs: u64) {
    // Create a new spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("/|\\- ") // Spinner animation characters
            .template("{spinner:.blue} {msg}") // Display template
            .unwrap(),
    );
    spinner.set_message(text); // Pass String directly
    spinner.enable_steady_tick(Duration::from_millis(100)); // Update interval

    // Simulate some work
    thread::sleep(Duration::from_secs(duration_secs));

    // Finish the spinner
    spinner.finish_with_message("Done!");
}
