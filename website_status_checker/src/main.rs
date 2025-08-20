use chrono::Utc;
use std::time::Duration;
use website_status_checker::{Config, Monitor};


fn main() {
    // Example list of URLs to monitor.
    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://www.example.com".to_string(),
        "https://www.google.com".to_string(),
    ];

    // Basic configuration
    let config = Config{
        workers: 4,
        timeout: Duration::from_secs(5),
        max_retries:2,
        repeat_interval: Some(Duration::from_secs(30)), // periodic monitoring      
    };

    let monitor = Monitor::new(config);

    // Run the first cycle immediately
    let results = monitor.run_once(&urls);
    for r in results {
        match r.status {
            Ok(code) => println!(
                "[{}] {} => {} ({} ms)",
                r.timestamp,
                r.url,
                code,
                r.response_time.as_millis()
            ),
            Err(err) => println!("[{}] {} => ERROR: {}", r.timestamp, r.url, err),
        }
    }

    // If we want to keep monitoring forever:
    // let all = monitor.run_periodic(&urls);
}
