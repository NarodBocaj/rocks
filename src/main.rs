use clap::Parser;

mod tickers;

/// Simple program to scrape stock price
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Enter ticker for price info
    #[arg(short, long)]
    ticker: String,
}

fn main() {
    let args = Args::parse();

    stock_price(&args.ticker);

    
}


fn stock_price(ticker: &str) {
    println!("Ticker: {}", ticker);

    // if tickers::exchanges::AMEX.contains(&ticker) || tickers::exchanges::NASDAQ.contains(&ticker) || tickers::exchanges::NYSE.contains(&ticker){//this currently is preventing ETFs
    //     scrape(ticker);
    // }
    // else{
    //     println!("{} is not a valid ticker", ticker);
    // }
    scrape(ticker);

}

fn help() {
    println!("Functions \n price TICKER");
}

fn scrape(ticker: &str) {
    let url = "https://finance.yahoo.com/quote/".to_owned() + ticker; //+ "p=" + ticker + "&.tsrc=fin-srch";

    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    
    let price_finder = scraper::Selector::parse("fin-streamer[value]").unwrap();

    let stock_name_selector = scraper::Selector::parse("h1.D\\(ib\\).Fz\\(18px\\)").unwrap();

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
}