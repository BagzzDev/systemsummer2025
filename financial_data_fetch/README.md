# Rust Price Fetcher Assignment

A simple Rust application that periodically fetches and records the pricing data of **Bitcoin**, **Ethereum**, and the **S&P 500 index**.  
It uses public APIs to get real-time data, saves the prices to separate text files, and repeats the process every 10 seconds.

---

## 📌 Features
- **Struct Creation**:  
  Three dedicated structs: `Bitcoin`, `Ethereum`, and `SP500`.
- **Pricing Trait**:  
  Defines a common interface (`fetch_price` and `save_to_file`) for all assets.
- **HTTP Requests**:  
  Uses [`ureq`](https://docs.rs/ureq/latest/ureq/) to fetch JSON data from public APIs.
- **JSON Parsing**:  
  Uses [`serde`](https://serde.rs/) for parsing API responses.
- **Data Storage**:  
  Saves fetched prices into separate `.txt` files inside a `data/` folder.
- **Periodic Execution**:  
  Runs indefinitely, fetching data every 10 seconds.

---

## 📂 Project Structure
```
project_root/
├─ src/
│  ├─ main.rs        # Main loop & application entry point
│  ├─ bitcoin.rs     # Bitcoin struct + Pricing trait implementation
│  ├─ ethereum.rs    # Ethereum struct + Pricing trait implementation
│  ├─ sp500.rs       # SP500 struct + Pricing trait implementation
│  └─ pricing.rs     # Pricing trait definition
├─ Cargo.toml        # Project dependencies
└─ data/             # Output files (created automatically)
   ├─ bitcoin.txt
   ├─ ethereum.txt
   └─ sp500.txt
```
---

## ⚙️ How It Works
1. **Initialization**  
   - Creates instances of `Bitcoin`, `Ethereum`, and `SP500`.
   - Stores them in a vector.

2. **Main Loop**  
   - For each asset:
     - Calls `fetch_price()` to get the latest price.
     - Calls `save_to_file()` to append the price to the corresponding `.txt` file.
   - Waits **10 seconds** and repeats.

3. **File Output**  
   - Prices are appended line-by-line to their respective `.txt` files in the `data/` folder.