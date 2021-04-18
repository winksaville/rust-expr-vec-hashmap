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

fn create_hash_map(symbols: &Vec<Symbol>) -> HashMap<&str, &Symbol> {
    let mut hm = HashMap::<&str, &Symbol>::new();
    for sym in symbols {
        hm.insert(&sym.name, &sym);
    }

    hm
}

fn main() {
    let mut symbols = Vec::<Symbol>::new();

    symbols.append(&mut vec![
        Symbol::new("BTCUSD".to_string()),
        Symbol::new("BNBUSD".to_string()),
    ]);
    //println!("symbols={:#?}", symbols);

    let symbols_hm = create_hash_map(&symbols);
    //println!("symbols_hm={:#?}", symbols_hm);
    println!("BTCUSD={:#?}", symbols_hm.get("BTCUSD"));
    println!("BNBUSD={:#?}", symbols_hm.get("BNBUSD"));
    println!("XYZUSD={:#?}", symbols_hm.get("XYZUSD"));
}
