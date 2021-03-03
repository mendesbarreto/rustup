struct Asset {
    name: String,
    symbol: String,
}

impl Asset {
    fn new(name: String, symbol: String) -> Asset {
        Asset {
            name: name,
            symbol: symbol,
        }
    }

    fn to_string(&self) -> String {
        format!("Asset: {}, Symbol: {}", self.name, self.symbol)
    }

    fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn dump(value: &String) {
    println!("{}", value);
}

fn main() {
    let value = Asset::new("Bitcoin".to_string(), "BTC".to_string());
    println!("{}", value.to_string());
}
