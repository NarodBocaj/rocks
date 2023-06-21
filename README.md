# <center>rocks</center>
Simple command line tool that scrapes Yahoo Finance to return stock information based on ticker or a company name search. 

# Quick Start
> ./target/release rocks NVDA  

Prints:   

>Ticker: NVDA  
>Stock Name: NVIDIA Corporation (NVDA)  
>Price: 401.11 | Daily Change: 11.64 | Pct Change 2.991%  


# Usage:
  
>rocks [OPTIONS] QUERY

  

Options:
| Flag    |  Description |
| ------- | ------------ |
| --name  | Force a search assuming your query is a company name |
| --ticker| Force a search assuming your query is a ticker |
| --week-range-52     | Print 52 week range of stock price |
| --mkt-cap           | Print current market cap |
| --pe-ratio          | Print current price to earnings ratio |
| --eps               | Print current earnings per share |
| --help              | Print help |
| --version           | Print version |

# Notes
* The default usage of rocks QUERY checks the QUERY to see if it is in a list of ~11,000 US stock and ETF tickers. If QUERY is not found, a trie based search is done assuming a company name was entered. So if searching a ticker that is on Yahoo Finance but not a US Symbol the ==--ticker== flag is your friend.
