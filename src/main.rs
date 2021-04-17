use std::collections::HashMap;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol {
            name,
        }
    }
}

fn main() {
    let mut symbols = Vec::<Symbol>::new();

    symbols.append(&mut vec![
        Symbol::new("BTCUSD".to_string()),
        Symbol::new("BNBUSD".to_string()),
    ]);
    println!("symbols={:#?}", symbols);

    let mut symbols_hm = HashMap::<&str, &Symbol>::new();
    symbols_hm.insert(&symbols[0].name, &symbols[0]);
    symbols_hm.insert(&symbols[1].name, &symbols[1]);
    println!("symbols_hm={:#?}", symbols_hm);
}
