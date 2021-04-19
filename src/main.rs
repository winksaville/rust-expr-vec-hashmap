use std::collections::HashMap;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol { name }
    }
}

fn create_hash_map(symbols: &[Symbol]) -> HashMap<&str, &Symbol> {
    let mut hm = HashMap::<&str, &Symbol>::new();
    for sym in symbols {
        hm.insert(&sym.name, &sym);
    }

    hm
}

#[derive(Debug)]
pub struct ExchangeInfo {
    pub server_time: u64,
    pub symbols: Vec<Symbol>,
}

fn main() {
    let mut symbols = Vec::<Symbol>::new();

    symbols.append(&mut vec![
        Symbol::new("BTCUSD".to_string()),
        Symbol::new("BNBUSD".to_string()),
    ]);
    //println!("symbols={:#?}", symbols);

    let ei = ExchangeInfo {
        server_time: 0,
        symbols: vec![],
    };
    println!("ei={:#?}", ei);

    let symbols_hm = create_hash_map(&symbols);
    //println!("symbols_hm={:#?}", symbols_hm);
    println!("BTCUSD={:#?}", symbols_hm.get("BTCUSD"));
    println!("BNBUSD={:#?}", symbols_hm.get("BNBUSD"));
    println!("XYZUSD={:#?}", symbols_hm.get("XYZUSD"));
}
