fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 3 {
        // println!("Usage: {} greet [name]", args[0]);
        // println!("Usage: {} info", args[0]);
        println!("Invalid Syntax");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "stock_price" => stock_price(&args[2]),
        "help" => help(),
        _ => println!("Invalid command: {}", command),
    }


}


fn stock_price(ticker: &str) {
    println!("Ticker: {}", ticker);
    scrape(ticker);
}


fn help() {
    println!("Here are the functions you can use");
}

fn scrape(ticker: &str) {
    let url = "https://finance.yahoo.com/quote/".to_owned() + ticker; //+ "p=" + ticker + "&.tsrc=fin-srch";

    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let price_finder = scraper::Selector::parse("fin-streamer[value]").unwrap();

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

    println!("Price: {} | Daily Change: {} | Pct Change {}%", price_info[0], price_info[1], (price_info[2].parse::<f64>().unwrap() * 100.0).to_string());
}