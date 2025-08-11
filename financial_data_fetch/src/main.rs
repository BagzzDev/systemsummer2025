mod pricing;
mod bitcoin;
mod ethereum;
mod sp_500;

use pricing::Pricing;
use bitcoin::Bitcoin;
use ethereum::Ethereum;
use sp_500::SP500;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create assets and store them as trait objects so we can treat uniformly.
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: 0.0 }),
        Box::new(Ethereum { price: 0.0 }),
        Box::new(SP500 { price: 0.0 }),
    ];

    loop {
        for asset in assets.iter_mut() {
            if let Err(e) = asset.fetch_price() {
                eprintln!("Fetch error: {}", e);
                continue;   // don't try to save if we couldn't fetch
            }
            if let Err(e) = asset.save_to_file() {
                eprintln!("Save error: {}", e);
            }
        }
        println!("Updated prices. Sleeping 10 seconds...");
        thread::sleep(Duration::from_secs(10));
    }
}
