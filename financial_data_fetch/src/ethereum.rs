use serde::Deserialize;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use crate::pricing::Pricing;

// Private struct used for JSON parsing of CoinGecko's response for Ethereum.
#[derive(Debug, Deserialize)]
struct EthereumPriceResponse {
    ethereum: CurrencyData,
}

// Private struct for holding currency price data in USD.
#[derive(Debug, Deserialize)]
struct CurrencyData {
    usd: f64,
}

// Public struct representing the Ethereum asset.
pub struct Ethereum {
    pub price: f64,
}

// Implementation of the `Pricing` trait for Ethereum.
impl Pricing for Ethereum {
    // Fetches Ethereum price in USD from CoinGecko's public API.
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let resp: EthereumPriceResponse = ureq::get(
            "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd",
        )
        .call()?
        .into_json()?;
        self.price = resp.ethereum.usd;
        Ok(())
    }

    // Saves the latest Ethereum price to `data/ethereum.txt`.
    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all("data")?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data/ethereum.txt")?;
        writeln!(file, "{}", self.price)?;
        Ok(())
    }
}