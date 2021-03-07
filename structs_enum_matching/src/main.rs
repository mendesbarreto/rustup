use std::fmt;

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

trait Print {
    fn print(&self);
}

impl Print for i32 {
    fn print(&self) {
        print!("{}", self);
    }
}

impl Print for Asset {
    fn print(&self) {
        print!("{}", self.to_string());
    }
}

impl fmt::Debug for Asset {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_string())
    }
}

fn main() {
    let asset = Asset::new("Bitcoin".to_string(), "BTC".to_string());
    print!("{:?}", asset);
}
