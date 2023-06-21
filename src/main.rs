use clap::Parser;
use trie_rs::{TrieBuilder, Trie};
use std::str;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use csv::Reader;

// mod tickers;

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
            stock_price(&args.query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps);
        }
        
        else if args.name {
            find_ticker(& ticker_map, & trie, &args.query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps);
        }
        
        else if ticker_hs.contains(&args.query){//checks if what is being searched is a ticker or a company name
            stock_price(&args.query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps);
        }
        else{
            find_ticker(& ticker_map, & trie, &args.query, args.week_range_52, args.mkt_cap, args.pe_ratio, args.eps);
        }
    }
    // if !args.name.is_empty() {
    //     find_ticker(& ticker_map, & trie, &args.name);
    // }
    
}


fn stock_price(ticker: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool) {
    println!("Ticker: {}", ticker);

    // if tickers::exchanges::AMEX.contains(&ticker) || tickers::exchanges::NASDAQ.contains(&ticker) || tickers::exchanges::NYSE.contains(&ticker){//this currently is preventing ETFs
    //     scrape(ticker);
    // }
    // else{
    //     println!("{} is not a valid ticker", ticker);
    // }
    scrape(ticker, week_range_52, mkt_cap, pe_ratio, eps);

}

fn scrape(ticker: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool) {
    let url = "https://finance.yahoo.com/quote/".to_owned() + ticker; //+ "p=" + ticker + "&.tsrc=fin-srch";

    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    
    let price_finder = scraper::Selector::parse("fin-streamer[value]").unwrap();//yahoo finance class that contains lots of prices info on the page

    let stock_name_selector = scraper::Selector::parse("h1.D\\(ib\\).Fz\\(18px\\)").unwrap();//yahoo finance html element that has stock price

    if let Some(element) = document.select(&stock_name_selector).next() {
        let stock_name = element.text().collect::<Vec<_>>().join("");
        println!("Stock Name: {}", stock_name);
    }

    let mut ticker_count = 0;
    let mut price_info = Vec::new();

    for element in document.select(&price_finder){
        if let Some(symbol) = element.value().attr("data-symbol") {
            if symbol == ticker{
                ticker_count += 1;
            }
        }
        if let Some(price) = element.value().attr("value") {
            if ticker_count > 0 && ticker_count <= 3{
                price_info.push(price);
            }
        }
    }
    if price_info.len() < 3{
        println!("Invalid Ticker: {}", ticker);
        return
    }
    println!("Price: {} | Daily Change: {:.5} | Pct Change {:.5}%", price_info[0], price_info[1], (price_info[2].parse::<f64>().unwrap() * 100.0).to_string());


    if week_range_52{
        let selector = scraper::Selector::parse("td[data-test=FIFTY_TWO_WK_RANGE-value]").unwrap();

        if let Some(element) = document.select(&selector).next() {
            let value = element.inner_html();
            println!("52 Week Range: {}", value);
        }
    }

    if mkt_cap{
        let selector = scraper::Selector::parse("td[data-test=MARKET_CAP-value]").unwrap();

        if let Some(element) = document.select(&selector).next() {
            let value = element.inner_html();
            println!("Market Cap: {}", value);
        }
    }

    if pe_ratio{
        let selector = scraper::Selector::parse("td[data-test=PE_RATIO-value]").unwrap();

        if let Some(element) = document.select(&selector).next() {
            let value = element.inner_html();
            println!("PE Ratio: {}", value);
        }
    }

    if eps{
        let selector = scraper::Selector::parse("td[data-test=EPS_RATIO-value]").unwrap();

        if let Some(element) = document.select(&selector).next() {
            let value = element.inner_html();
            println!("EPS: {}", value);
        }
    }
}

//function to make a trie and hashmap from the filtered data
fn make_trie_hm(ticker_map: &mut HashMap<String, String>, builder: &mut TrieBuilder<u8>, ticker_hs: &mut HashSet<String>) -> Result<(), Box<dyn std::error::Error>> {
    let equities_file = File::open("./filtered_data/equities.csv")?;
    let etfs_file = File::open("./filtered_data/etfs.csv")?;
    
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
fn find_ticker(ticker_map: & HashMap<String, String>, trie: & Trie<u8>, company_name: &str, week_range_52: bool, mkt_cap: bool, pe_ratio: bool, eps: bool) -> () {
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

    println!("Search produced the following results: {:?} \n", results_in_str);
    if !results_in_str.is_empty(){
        scrape(ticker_map.get(results_in_str[0]).map(|s| s.as_str()).unwrap_or(""), week_range_52, mkt_cap, pe_ratio, eps);
    }
    else{
        println!("No results found");
    }
}