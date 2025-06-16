# <center>rocks</center>
Simple command line tool that brings Yahoo Finance stock info to your terminal. 

# Installation

## Mac (via Homebrew)
```bash

brew tap NarodBocaj/rocks
```
```bash
brew install rocks
```

## Linux
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

> Apple Inc. (AAPL)
> Price: 198.42  | Daily Change: +1.97  | Pct Change: +(1.00%)
> After Close Price: 197.99

Get multiple statistics:
```bash
rocks AXP -m -p -d
```

> American Express Company (AXP)
> Price: 294.22  | Daily Change: +6.43  | Pct Change: +(2.23%)
> After Close Price: 294.75
> 
> Market Cap: 206.127B
> PE Ratio: 20.54
> Day's Range: 290.02 - 296.45

Search by company name:
```bash
rocks "apple"
```

> Search Results
> -------------
> [0]  Company: apple hospitality reit, inc.             | Ticker: APLE
> [1]  Company: apple inc.                               | Ticker: AAPL
> 
> Enter the number of your choice (0-1):

# Notes
* The default usage of rocks ```<QUERY>``` checks the ```<QUERY>``` to see if it is in a list of ~11,000 US stock and ETF tickers. If ```<QUERY>``` is not found, a trie based search is done assuming a company name was entered. So if searching a ticker that is on Yahoo Finance but not a US Symbol the ```--ticker``` flag is your friend.
* When searching by company name, you'll be presented with a list of matching companies to choose from.