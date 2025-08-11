use serde::Deserialize;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use crate::pricing::Pricing;

// Private structs matching the JSON structure from Yahoo Finance's API for SPY (S&P 500 ETF).
#[derive(Debug, Deserialize)]
struct QuoteResponse {
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart {
    result: Option<Vec<ResultItem>>,
}

#[derive(Debug, Deserialize)]
struct ResultItem {
    meta: Meta,
}

#[derive(Debug, Deserialize)]
struct Meta {
    regularMarketPrice: f64,
}

// Public struct representing the S&P 500 (via SPY ETF).
pub struct SP500 {
    pub price: f64,
}

// Implementation of the `Pricing` trait for SP500.
impl Pricing for SP500 {
    // Fetches the latest SP500 price from Yahoo Finance's public API
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/SPY?interval=2m&range=1d";
        let resp: QuoteResponse = ureq::get(url).call()?.into_json()?;
        if let Some(res) = resp.chart.result.and_then(|mut v| v.pop()) {
            self.price = res.meta.regularMarketPrice;
            Ok(())
        } else {
            Err("Failed to parse SPY price".into())
        }
    }

    // Saves the latest SP500 price to `data/sp500.txt`.
    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        create_dir_all("data")?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data/sp500.txt")?;
        writeln!(file, "{}", self.price)?;
        Ok(())
    }
}