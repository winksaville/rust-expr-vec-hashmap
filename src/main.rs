use chrono::prelude::*;
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
    pub server_time: i64,
    pub symbols: Vec<Symbol>,
}

impl Default for ExchangeInfo {
    fn default() -> ExchangeInfo {
        ExchangeInfo {
            server_time: 0,
            symbols: vec![],
        }
    }
}

impl ExchangeInfo {
    pub fn new() -> ExchangeInfo {
        let mut ei = ExchangeInfo::default();
        ei.update_server_time();
        ei
    }

    pub fn update_server_time(&mut self) -> &Self {
        self.server_time = Utc::now().timestamp_millis();
        self
    }

    pub fn add_symbol(&mut self, sym: Symbol) -> &Self {
        self.symbols.push(sym);
        self
    }
}

fn main() {
    let mut ei = ExchangeInfo::new();
    println!("ei={:#?}", ei);
    ei.add_symbol(Symbol::new("BTCUSD".to_string()));
    ei.add_symbol(Symbol::new("BNBUSD".to_string()));
    println!("ei={:#?}", ei);

    let symbols_hm = create_hash_map(&ei.symbols);
    //println!("symbols_hm={:#?}", symbols_hm);
    println!("BTCUSD={:#?}", symbols_hm.get("BTCUSD"));
    println!("BNBUSD={:#?}", symbols_hm.get("BNBUSD"));
    println!("XYZUSD={:#?}", symbols_hm.get("XYZUSD"));
}
