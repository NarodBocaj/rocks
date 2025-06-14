# <center>rocks</center>
Simple command line tool that scrapes Yahoo Finance to return stock information based on ticker or a company name search. 

# Installation

## Mac/Linux
1. Clone the repository:
```bash
git clone https://github.com/NarodBocaj/rocks
cd rocks
```

2. Build the project:
```bash
cargo build --release
```

3. Add to your PATH:
```bash
sudo ln -sf "$(pwd)/target/release/rocks" /usr/local/bin/rocks
sudo cp filtered_data/equities.csv filtered_data/etfs.csv /usr/local/bin/
```


# Quick Start
```bash
rocks NVDA
```  

Prints:   

>Ticker: NVDA  
>Stock Name: NVIDIA Corporation (NVDA)  
>Price: 401.11 | Daily Change: 11.64 | Pct Change 2.991%  

# Usage:
  
```rocks [OPTIONS] <QUERY>```

Options:
| Flag    |  Description |
| ------- | ------------ |
| --name  | Force a search assuming your query is a company name |
| --ticker| Force a search assuming your query is a ticker |
| --week-range-52     | Print 52 week range of stock price |
| --mkt-cap           | Print current market cap |
| --pe-ratio          | Print current price to earnings ratio |
| --eps               | Print current earnings per share |
| --day-range         | Print the day's trading range (low - high) |
| --help              | Print help |
| --version           | Print version |

# Examples

Basic usage:
```bash
rocks AAPL
```

Get multiple statistics:
```bash
rocks AAPL -m -p -d
```

Search by company name:
```bash
rocks "apple inc"
```

# Notes
* The default usage of rocks ```<QUERY>``` checks the ```<QUERY>``` to see if it is in a list of ~11,000 US stock and ETF tickers. If ```<QUERY>``` is not found, a trie based search is done assuming a company name was entered. So if searching a ticker that is on Yahoo Finance but not a US Symbol the ```--ticker``` flag is your friend.
* When searching by company name, you'll be presented with a list of matching companies to choose from.
* The program requires the `equities.csv` and `etfs.csv` files to be in the same directory as the executable or in a `filtered_data` subdirectory.
