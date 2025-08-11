pub trait Pricing {
    fn fetch_price(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>>;
}