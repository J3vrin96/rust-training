#[derive(Debug)]
enum UsState {
    Alabama,
    Montana
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{six} {none}");
}

fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("A quarter from {state:?}");
            25
        }
    }
}

 fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}


