use clap::Parser;
use trie_rs::{TrieBuilder, Trie};
use std::str;
use std::env;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use csv::Reader;

mod debug;

/// Simple program to scrape stock price
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Enter ticker or company name for price info
    // #[arg(short, long)]
    query: String,

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

    /// Print PE Ration
    #[arg(short, long, action)]
    pe_ratio: bool,

    /// Print earning per share
    #[arg(short, long, action)]
    eps: bool,

    /// Print day's trading range
    #[arg(short, long, action)]
    day_range: bool,
}

fn main() {
    let args = Args::parse();

    let mut ticker_map: HashMap<String, String> = HashMap::new(); //maps company name to ticker
    let mut builder: TrieBuilder<u8> = TrieBuilder::new();
    let mut ticker_hs: HashSet<String> = HashSet::new();
    make_trie_hm(&mut ticker_map, &mut builder, &mut ticker_hs);

    let trie = builder.build();

    if !args.query.is_empty() {
        if args.name && args.ticker {
            println!("Ticker flag and name flag cannot be used together");
        }
        
        else if args.ticker {
            stock_price(&args.query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps, args.day_range);
        }
        
        else if args.name {
            find_ticker(& ticker_map, & trie, &args.query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps, args.day_range);
        }
        
        else if ticker_hs.contains(&args.query){//checks if what is being searched is a ticker or a company name
            stock_price(&args.query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps, args.day_range);
        }
        else{
            find_ticker(& ticker_map, & trie, &args.query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps, args.day_range);
        }
    }
    // if !args.name.is_empty() {
    //     find_ticker(& ticker_map, & trie, &args.name);
    // }
    
}


fn stock_price(ticker: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool, day_range: bool) {
    println!("Ticker: {}", ticker);

    // if tickers::exchanges::AMEX.contains(&ticker) || tickers::exchanges::NASDAQ.contains(&ticker) || tickers::exchanges::NYSE.contains(&ticker){//this currently is preventing ETFs
    //     scrape(ticker);
    // }
    // else{
    //     println!("{} is not a valid ticker", ticker);
    // }
    scrape(ticker, week_range_52, mkt_cap, pe_ratio, eps, day_range);

}

fn scrape(ticker: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool, day_range: bool) {
    let url = "https://finance.yahoo.com/quote/".to_owned() + ticker;

    // Create a client with a User-Agent header
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()
        .unwrap();

    // Add better error handling for the request
    let response = match client.get(&url).send() {
        Ok(resp) => match resp.text() {
            Ok(text) => text,
            Err(e) => {
                println!("Error reading response: {}", e);
                return;
            }
        },
        Err(e) => {
            println!("Error fetching URL: {}", e);
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

    // Try to find the quote-price section first
    let quote_section = scraper::Selector::parse("section[data-testid='quote-price']").unwrap();
    if let Some(section) = document.select(&quote_section).next() {
        // Get the current price using the exact selector from the HTML
        let price_selector = scraper::Selector::parse("span[data-testid='qsp-price']").unwrap();
        let price = section.select(&price_selector).next()
            .map(|e| e.text().collect::<Vec<_>>().join(""))
            .unwrap_or_else(|| "N/A".to_string());

        // Get the price change using the exact selector
        let change_selector = scraper::Selector::parse("span[data-testid='qsp-price-change']").unwrap();
        let change = section.select(&change_selector).next()
            .map(|e| e.text().collect::<Vec<_>>().join(""))
            .unwrap_or_else(|| "N/A".to_string());

        // Get the percent change using the exact selector
        let percent_selector = scraper::Selector::parse("span[data-testid='qsp-price-change-percent']").unwrap();
        let percent = section.select(&percent_selector).next()
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

    // Try to find the CSV files in the following locations:
    // 1. In the same directory as the executable
    // 2. In a 'filtered_data' subdirectory of the executable's directory
    // 3. In the current working directory
    // 4. In a 'filtered_data' subdirectory of the current working directory
    
    let possible_paths = [
        exe_dir.join("equities.csv"),
        exe_dir.join("etfs.csv"),
        exe_dir.join("filtered_data/equities.csv"),
        exe_dir.join("filtered_data/etfs.csv"),
        std::env::current_dir()?.join("equities.csv"),
        std::env::current_dir()?.join("etfs.csv"),
        std::env::current_dir()?.join("filtered_data/equities.csv"),
        std::env::current_dir()?.join("filtered_data/etfs.csv"),
    ];

    let mut equities_path = None;
    let mut etfs_path = None;

    for path in possible_paths {
        if path.exists() {
            if path.file_name().unwrap_or_default() == "equities.csv" {
                equities_path = Some(path);
            } else if path.file_name().unwrap_or_default() == "etfs.csv" {
                etfs_path = Some(path);
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
fn find_ticker(ticker_map: & HashMap<String, String>, trie: & Trie<u8>, company_name: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool, day_range: bool) -> () {
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
                scrape(ticker, week_range_52, mkt_cap, pe_ratio, eps, day_range);
            }
        },
        _ => println!("Invalid selection. Please enter a number between 0 and {}", results_in_str.len() - 1)
    }
}