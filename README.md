# Kraken Trades
The main goal of this project is to create a CLI tool for downloading historical trades from the Kraken cryptocurrency exchange. The current implementation only supports requests for "futures" trades.

Features:
- Store trades in a CSV output file
- Request trades for predefined time periods: `this-week`, `this-month`, `this-year`, `last-day`, `last-year`

Using this tool requires an API key, which must be generated from your Kraken account. To do this, please refer to the [How to create an API key on Kraken Pro
](https://support.kraken.com/hc/en-us/articles/how-to-create-an-api-key-on-kraken-pro). 
Please note: access **Futures** data, the API key must have at least **Read-only** permission for the General API. Once generated, store the credentials in a JSON file (see `kraken_futures_key_example.json` for the required format).

# Run
To run type in console:
```
cargo run -- --kraken-credentials /home/kraken-trades/kraken_futures_key_example.json --csv-output /home/kraken-trades/kraken_futures_trades.csv --period this-month
```

for more details:
```
cargo run -- --help
```