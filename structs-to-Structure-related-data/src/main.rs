
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: false
    }
}


#[derive(Debug)]
struct Asset {
    name: String,
    symbol: String,
    amount: u64,
}

fn build_asset(name: String, symbol: String) -> Asset {
    Asset {
        name,
        symbol,
        amount: 100
    }
}

fn main() {
    let user = build_user(
        "super@email.com".to_string(),
        "admin".to_string()
    );

    let asset = build_asset("Bitcoin".to_string(), "BTC".to_string());

    println!("{:?}", user);
    println!("{:?}", asset);
}
