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

fn main() {
    for x in range(0.0, 1.0, 0.1) {
        println!("{:.1}", x);
    }
}
