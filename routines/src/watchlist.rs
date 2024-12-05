use std::{error::Error, fs};

pub fn watchlist_load(file: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(file)?)
}

pub fn watchlist_tradedates_for_ticker(path: &str, ticker: &str) -> Result<String, Box<dyn Error>> {
    let paths = fs::read_dir(path)?;

    let mut ret = String::new();
    for path in paths {
        let path = path?.path();

        let file_name = path.file_name().unwrap().to_str().unwrap();

        if !path.is_file() || !file_name.contains(ticker) {
            continue;
        }

        ret.push_str(&file_name[0..10]);
        ret.push_str("\n");
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watchlist_load() {
        let result = watchlist_load("../data/watchlist.txt");
        println!("result: {result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_watchlist_trade_date_for_ticker() {
        let result = watchlist_tradedates_for_ticker("../data/market", "HUT");
        println!("result: {result:?}");
        assert!(result.is_ok());
    }
}
