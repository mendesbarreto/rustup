use std::fmt;

struct Asset {
    name: String,
    symbol: Symbol,
}

impl Asset {
    fn new(name: String, symbol: Symbol) -> Asset {
        Asset {
            name: name,
            symbol: symbol,
        }
    }

    fn to_string(&self) -> String {
        format!("Asset: {}, Symbol: {}", self.name, self.symbol.as_str())
    }

    fn set_symbol(&mut self, symbol: Symbol) {
        self.symbol = symbol;
    }
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl fmt::Debug for Asset {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_string())
    }
}

struct FRange {
    val: f64,
    end: f64,
    incr: f64,
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {
        val: x1,
        end: x2,
        incr: skip,
    }
}

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.val;
        if result >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(result)
        }
    }
}

fn sqrt<T>(x: T) -> T::Output
where
    T: std::ops::Mul + Copy,
{
    x * x
}

enum Symbol {
    Btc,
    Dot,
    Eth,
}

impl Symbol {
    fn as_str(&self) -> &'static str {
        match *self {
            Symbol::Btc => "BTC",
            Symbol::Dot => "DOT",
            Symbol::Eth => "ETH",
        }
    }
}

fn main() {
    let bitcoin = Asset::new("Bitcoin".to_string(), Symbol::Btc);
    let shitcoin = Asset {
        name: "Shitcoin".to_string(),
        ..bitcoin
    };

    print!("{:?}", shitcoin);
}
