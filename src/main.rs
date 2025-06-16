use clap::Parser;
use trie_rs::{TrieBuilder, Trie};
use std::str;
use std::env;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use csv::Reader;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

mod debug;

/// Simple program to scrape stock price
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Enter ticker or company name for price info
    #[arg(required = false)]
    query: Option<String>,

    /// Force company name based search
    #[arg(short, long, action)]
    name: bool,

    /// Force ticker based search
    #[arg(short, long, action)]
    ticker: bool,

    /// Print 52 week range of price
    #[arg(short, long, action)]
    week_range_52: bool,

    /// Print market cap
    #[arg(short, long, action)]
    mkt_cap: bool,

    /// Print PE Ratio
    #[arg(short, long, action)]
    pe_ratio: bool,

    /// Print earning per share
    #[arg(short, long, action)]
    eps: bool,

    /// Print day's trading range
    #[arg(short, long, action)]
    day_range: bool,

    /// Print company information
    #[arg(short, long, action)]
    information: bool,
}

fn main() {
    let args = Args::parse();

    // If no query is provided, just return
    let query = match args.query {
        Some(q) => q,
        None => {
            println!("No query provided. Please provide a ticker or company name.");
            return;
        }
    };

    // Only load data if we're actually going to perform a query
    let mut ticker_map: HashMap<String, String> = HashMap::new();
    let mut builder: TrieBuilder<u8> = TrieBuilder::new();
    let mut ticker_hs: HashSet<String> = HashSet::new();
    
    if let Err(e) = make_trie_hm(&mut ticker_map, &mut builder, &mut ticker_hs) {
        println!("Error loading ticker data: {}", e);
        return;
    }

    let trie = builder.build();

    if args.name && args.ticker {
        println!("Ticker flag and name flag cannot be used together");
    }
    else if args.ticker {
        stock_price(&query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps, args.day_range, args.information);
    }
    else if args.name {
        find_ticker(& ticker_map, & trie, &query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps, args.day_range, args.information);
    }
    else if ticker_hs.contains(&query) {
        stock_price(&query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps, args.day_range, args.information);
    }
    else {
        find_ticker(& ticker_map, & trie, &query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps, args.day_range, args.information);
    }
}

fn stock_price(ticker: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool, day_range: bool, information: bool) {
    println!("Ticker: {}", ticker);

    // if tickers::exchanges::AMEX.contains(&ticker) || tickers::exchanges::NASDAQ.contains(&ticker) || tickers::exchanges::NYSE.contains(&ticker){//this currently is preventing ETFs
    //     scrape(ticker);
    // }
    // else{
    //     println!("{} is not a valid ticker", ticker);
    // }
    scrape(ticker, week_range_52, mkt_cap, pe_ratio, eps, day_range, information);

}

fn scrape(ticker: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool, day_range: bool, information: bool) {
    let url = "https://finance.yahoo.com/quote/".to_owned() + ticker;
    let max_retries = 3;
    let mut retry_count = 0;
    let mut response = None;

    // Create a client with proper headers
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    // Retry loop
    while retry_count < max_retries {
        match client.get(&url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Connection", "keep-alive")
            .header("Upgrade-Insecure-Requests", "1")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .header("Cache-Control", "max-age=0")
            .send() {
            Ok(resp) => match resp.text() {
                Ok(text) => {
                    response = Some(text);
                    break;
                },
                Err(e) => {
                    println!("Error reading response (attempt {}/{}): {}", retry_count + 1, max_retries, e);
                    retry_count += 1;
                    if retry_count < max_retries {
                        std::thread::sleep(std::time::Duration::from_secs(2));
                    }
                }
            },
            Err(e) => {
                println!("Error fetching URL (attempt {}/{}): {}", retry_count + 1, max_retries, e);
                retry_count += 1;
                if retry_count < max_retries {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        }
    }

    let response = match response {
        Some(text) => text,
        None => {
            println!("Failed to fetch data after {} attempts. Please try again later.", max_retries);
            return;
        }
    };

    let document = scraper::Html::parse_document(&response);
    
    // First check if the page exists
    let error_selector = scraper::Selector::parse("div.error-container").unwrap();
    if document.select(&error_selector).next().is_some() {
        println!("Invalid Ticker: {} - Page not found", ticker);
        return;
    }

    // Get company name
    let name_selector = scraper::Selector::parse("h1.yf-4vbjci").unwrap();
    if let Some(name) = document.select(&name_selector).next() {
        println!("\n{}", name.text().collect::<Vec<_>>().join(""));
    }

    // Try to find the price-statistic section first
    let price_section = scraper::Selector::parse("section[data-testid='price-statistic']").unwrap();
    if let Some(section) = document.select(&price_section).next() {
        // Get the current price
        let price = section.select(&scraper::Selector::parse("span[data-testid='qsp-price']").unwrap())
            .next()
            .map(|e| e.text().collect::<Vec<_>>().join(""))
            .unwrap_or_else(|| "N/A".to_string());

        // Get the price change
        let change = section.select(&scraper::Selector::parse("span[data-testid='qsp-price-change']").unwrap())
            .next()
            .map(|e| e.text().collect::<Vec<_>>().join(""))
            .unwrap_or_else(|| "N/A".to_string());

        // Get the percent change
        let percent = section.select(&scraper::Selector::parse("span[data-testid='qsp-price-change-percent']").unwrap())
            .next()
            .map(|e| e.text().collect::<Vec<_>>().join(""))
            .unwrap_or_else(|| "N/A".to_string());

        println!("Price: {} | Daily Change: {} | Pct Change: {}", 
            price, 
            change,
            percent
        );
    } else {
        println!("Invalid Ticker: {} - Could not find price information", ticker);
        return;
    }

    // Check for after-hours price
    let after_hours_selector = scraper::Selector::parse("span[data-testid='qsp-post-price']").unwrap();
    if let Some(after_hours) = document.select(&after_hours_selector).next() {
        println!("After Close Price: {}", after_hours.text().collect::<Vec<_>>().join(""));
    }

    // Get company information if requested
    if information {
        let info_selector = scraper::Selector::parse("p.yf-1ja4ll8").unwrap();
        if let Some(info) = document.select(&info_selector).next() {
            println!("\nCompany Information:");
            println!("{}", info.text().collect::<Vec<_>>().join(""));
        }
    }

    // Get additional statistics if requested
    let stats_section = scraper::Selector::parse("div[data-testid='quote-statistics']").unwrap();
    if let Some(section) = document.select(&stats_section).next() {
        // Get the command line arguments to determine flag order
        let args: Vec<String> = std::env::args().collect();
        
        // Create a vector of requested statistics with their selectors and labels
        let mut stats_to_show = Vec::new();
        
        // Check flags in the order they appear in the command line
        for arg in &args {
            match arg.as_str() {
                "-d" | "--day-range" if day_range => {
                    stats_to_show.push((
                        "Day's Range",
                        "fin-streamer[data-field='regularMarketDayRange']"
                    ));
                },
                "-w" | "--week-range-52" if week_range_52 => {
                    stats_to_show.push((
                        "52 Week Range",
                        "fin-streamer[data-field='fiftyTwoWeekRange']"
                    ));
                },
                "-m" | "--mkt-cap" if mkt_cap => {
                    stats_to_show.push((
                        "Market Cap",
                        "fin-streamer[data-field='marketCap']"
                    ));
                },
                "-p" | "--pe-ratio" if pe_ratio => {
                    stats_to_show.push((
                        "PE Ratio",
                        "fin-streamer[data-field='trailingPE']"
                    ));
                },
                "-e" | "--eps" if eps => {
                    stats_to_show.push((
                        "EPS",
                        "fin-streamer[data-field='trailingPE']"
                    ));
                },
                _ => continue,
            }
        }

        // Print the statistics in the order they were requested
        if stats_to_show.len() > 0{
            println!("\n");
        }
        for (label, selector) in stats_to_show {    
            let stat_selector = scraper::Selector::parse(selector).unwrap();
            if let Some(element) = section.select(&stat_selector).next() {
                if let Some(value) = element.value().attr("data-value") {
                    println!("{}: {}", label, value);
                }
            }
        }
    }
}

//function to make a trie and hashmap from the filtered data
fn make_trie_hm(ticker_map: &mut HashMap<String, String>, builder: &mut TrieBuilder<u8>, ticker_hs: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Get the executable's path
    let exe_path = std::env::current_exe()?;
    
    // Get the directory containing the executable
    let exe_dir = exe_path.parent().ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::Other,
            "Failed to get the parent directory of the executable.",
        )
    })?;

    // Check for ROCKS_DATA_DIR environment variable first
    let data_dir = if let Ok(dir) = std::env::var("ROCKS_DATA_DIR") {
        PathBuf::from(dir)
    } else {
        exe_dir.to_path_buf()
    };

    // Try to find the CSV files in the following locations:
    let possible_paths = [
        data_dir.join("equities.csv"),
        data_dir.join("etfs.csv"),
        exe_dir.join("equities.csv"),
        exe_dir.join("etfs.csv"),
        exe_dir.join("filtered_data/equities.csv"),
        exe_dir.join("filtered_data/etfs.csv"),
        std::env::current_dir()?.join("equities.csv"),
        std::env::current_dir()?.join("etfs.csv"),
        std::env::current_dir()?.join("filtered_data/equities.csv"),
        std::env::current_dir()?.join("filtered_data/etfs.csv"),
        PathBuf::from("/usr/local/share/rocks/equities.csv"),
        PathBuf::from("/usr/local/share/rocks/etfs.csv"),
        PathBuf::from("/opt/homebrew/share/rocks/equities.csv"),
        PathBuf::from("/opt/homebrew/share/rocks/etfs.csv"),
    ];

    let mut equities_path = None;
    let mut etfs_path = None;

    for path in possible_paths {
        if path.exists() {
            if path.file_name().unwrap_or_default() == "equities.csv" {
                equities_path = Some(path.clone());
            } else if path.file_name().unwrap_or_default() == "etfs.csv" {
                etfs_path = Some(path.clone());
            }
        }
    }

    let equities_path = equities_path.ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find equities.csv in any of the expected locations.",
        )
    })?;

    let etfs_path = etfs_path.ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find etfs.csv in any of the expected locations.",
        )
    })?;

    // Now, open the CSV files using the found paths
    let equities_file = File::open(equities_path)?;
    let etfs_file = File::open(etfs_path)?;
    
    let equities_reader = BufReader::new(equities_file);
    let etfs_reader = BufReader::new(etfs_file);
    
    let mut equities_csv_reader = Reader::from_reader(equities_reader);
    let mut etfs_csv_reader = Reader::from_reader(etfs_reader);
    
    for record in equities_csv_reader.records() {
        let record = record?;
        
        if let Some((first, second)) = record.get(0).and_then(|first| record.get(1).map(|second| (first, second))) {
            ticker_map.insert(second.to_owned().to_lowercase(), first.to_owned());
            builder.push(second.to_lowercase());
            ticker_hs.insert(first.to_string());
        }
    }
    
    for record in etfs_csv_reader.records() {
        let record = record?;
        
        if let Some((first, second)) = record.get(0).and_then(|first| record.get(1).map(|second| (first, second))) {
            ticker_map.insert(second.to_owned().to_lowercase(), first.to_owned());
            builder.push(second.to_lowercase());
            ticker_hs.insert(first.to_string());
        }
    }
    
    Ok(())
}

//function to find a ticker based on a company name
fn find_ticker(ticker_map: & HashMap<String, String>, trie: & Trie<u8>, company_name: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool, day_range: bool, information: bool) -> () {
    let company_name = company_name.to_lowercase();
    let mut temp_search = String::new();
    let mut last_result: Vec<Vec<u8>> = vec![vec![]];

    for c in company_name.chars(){
        temp_search.push(c);
        let results_in_u8s: Vec<Vec<u8>> = trie.predictive_search(&temp_search);

        if !results_in_u8s.is_empty() {
            last_result = results_in_u8s.clone();
        }
    }

    let results_in_str: Vec<&str> = last_result
        .iter()
        .map(|u8s| std::str::from_utf8(u8s).unwrap())
        .collect();

    if results_in_str.is_empty() {
        println!("No results found");
        return;
    }

    println!("\nSearch Results:");
    println!("---------------");
    for (i, company) in results_in_str.iter().enumerate() {
        if let Some(ticker) = ticker_map.get(*company) {
            println!("[{}]  Company: {:<40} | Ticker: {}", 
                i, 
                company, 
                ticker
            );
        }
    }
    println!("\nEnter the number of your choice (0-{}):", results_in_str.len() - 1);

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    
    match choice.trim().parse::<usize>() {
        Ok(index) if index < results_in_str.len() => {
            if let Some(ticker) = ticker_map.get(results_in_str[index]) {
                println!("\nSelected - Ticker: {}", ticker);
                scrape(ticker, week_range_52, mkt_cap, pe_ratio, eps, day_range, information);
            }
        },
        _ => println!("Invalid selection. Please enter a number between 0 and {}", results_in_str.len() - 1)
    }
}