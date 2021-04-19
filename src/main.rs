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

pub fn create_hash_map(symbols: &[Symbol]) -> HashMap<&str, &Symbol> {
    let mut hm = HashMap::<&str, &Symbol>::new();
    for sym in symbols {
        hm.insert(&sym.name, &sym);
    }

    hm
}

#[derive(Debug)]
pub struct ExchangeInfo<'e> {
    pub server_time: i64,
    pub symbols: Vec<Symbol>,
    symbols_hm: HashMap<&'e str, &'e Symbol>,
}

impl<'e> Default for ExchangeInfo<'e> {
    fn default() -> ExchangeInfo<'e> {
        ExchangeInfo {
            server_time: 0,
            symbols: vec![],
            symbols_hm: HashMap::<&str, &Symbol>::new(),
        }
    }
}

impl<'e> ExchangeInfo<'e> {
    pub fn new() -> ExchangeInfo<'e> {
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

    fn create_symbol_hash_map(&'e mut self) -> &Self {
        self.symbols_hm = create_hash_map(&self.symbols);
        self
    }

    fn get_symbol_via_hm(&self, name: &str) -> Option<&Symbol> {
        Some(*self.symbols_hm.get(name)?)
    }

    pub fn get_symbol(&'e mut self, name: &'e str) -> Option<&'e Symbol> {
        let sym = self.symbols_hm.get(name);
        if sym.is_none() && self.symbols_hm.is_empty() {
            Some(*self.create_symbol_hash_map().symbols_hm.get(name)?)
        } else {
            Some(*sym?)
        }
    }
}

fn main() {
    let mut mut_ei = ExchangeInfo::new();
    println!("mut_ei={:#?}", mut_ei);
    mut_ei.add_symbol(Symbol::new("BTCUSD".to_string()));
    mut_ei.add_symbol(Symbol::new("BNBUSD".to_string()));
    println!("mut_ei={:#?}", mut_ei);

    if true {
        // OK any number of times
        let ei = mut_ei.create_symbol_hash_map();
        println!("BTCUSD={:#?}", ei.get_symbol_via_hm("BTCUSD"));
        println!("BNBUSD={:#?}", ei.get_symbol_via_hm("BNBUSD"));
        println!("XYZUSD={:#?}", ei.get_symbol_via_hm("XYZUSD"));
    } else {
        // OK once
        println!("BNBUSD={:#?}", mut_ei.get_symbol("BNBUSD"));

        //   Below fails with compile error if used again! This is "less" convenient,
        //   but it makes things obvious that there is an initialization phase where
        //   `mut_ei` is mutable and not "fnished" and when it's complete and now can
        //   be referenced multiple times.
        // println!("BNBUSD={:#?}", mut_ei.get_symbol("BNBUSD"));
        //
        //   Here is the compile error for above:
        //    $ cargo run
        //       Compiling expr-vec-hashmap v0.1.0 (/home/wink/prgs/rust/projects/expr-vec-hashmap)
        //    error[E0499]: cannot borrow `mut_ei` as mutable more than once at a time
        //      --> src/main.rs:98:34
        //       |
        //    92 |         println!("BNBUSD={:#?}", mut_ei.get_symbol("BNBUSD"));
        //       |                                  ------ first mutable borrow occurs here
        //    ...
        //    98 |         println!("BNBUSD={:#?}", mut_ei.get_symbol("BNBUSD"));
        //       |                                  ^^^^^^
        //       |                                  |
        //       |                                  second mutable borrow occurs here
        //       |                                  first borrow later used here
        //
        //    error: aborting due to previous error
    }
}
