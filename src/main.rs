#[derive(Debug)]
pub struct Symbol {
    pub name: String,
}

fn main() {
    let mut symbols = Vec::<Symbol>::new();

    symbols.append(&mut vec![
        Symbol {
            name: "BTCUSD".to_string(),
        },
        Symbol {
            name: "BNBUSD".to_string(),
        },
    ]);
    println!("symbols={:#?}", symbols);
}
