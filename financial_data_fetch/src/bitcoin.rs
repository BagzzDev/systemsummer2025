use serde::Deserialize;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use crate::pricing::Pricing;

// Private struct used for JSON parsing of CoinGecko's response for Bitcoin.
// Matches the structure of: { "bitcoin": { "usd": <value> } }
#[derive(Debug, Deserialize)]
struct BitcoinPriceResponse {
    bitcoin: CurrencyData,
}

// Private struct for holding currency price data in USD.
#[derive(Debug, Deserialize)]
struct CurrencyData {
    usd: f64,
}

pub struct Bitcoin{
    pub price: f64,
}

// Implementation of the `Pricing` trait for Bitcoin.
impl Pricing for Bitcoin {
    // Fetches Bitcoin price in USD from CoinGecko's public API.
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let resp: BitcoinPriceResponse = ureq::get(
            "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd",
        )
        .call()?
        .into_json()?;
        self.price = resp.bitcoin.usd;
        Ok(())
    }

    // Saves the latest Bitcoin price to `data/bitcoin.txt`.
    // Automatically creates the `data` folder if it doesn't exist.
    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all("data")?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data/bitcoin.txt")?;
        writeln!(file, "{}", self.price)?;
        Ok(())
    }
}