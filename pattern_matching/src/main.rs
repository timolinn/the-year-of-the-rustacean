enum CryptoCurrencies {
    BitCoin,
    Ethereum,
    LiteCoin,
    BitCoinCash
}

fn value_balance(crypto: CryptoCurrencies) -> f32 {
    match crypto {
        CryptoCurrencies::BitCoin => 1.0,
        CryptoCurrencies::Ethereum => 0.8,
        CryptoCurrencies::LiteCoin => 0.5,
        CryptoCurrencies::BitCoinCash => 0.2,
    }
}

fn main() {
    let bal = value_balance(CryptoCurrencies::BitCoin);
    println!("{}", bal);

    // If lets are like match statement but with
    // One arm and the default
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
