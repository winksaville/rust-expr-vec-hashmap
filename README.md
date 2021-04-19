# Experiment with Vec and HashMap

Experiment with lazily creating a HashMap<k, v> from a Vec<v>
where k is-a field of v.

So it turns out trying to lazily create the hashmap is not
a good idea. To do so you must pass to `get_symbol` a mutable
reference to an ExchangeInfo. Any subsequent attempt to invoke
`get_symbol` will cause a compile error:

```
$ cargo build
   Compiling expr-vec-hashmap v0.1.0 (/home/wink/prgs/rust/projects/expr-vec-hashmap)
error[E0499]: cannot borrow `mut_ei` as mutable more than once at a time
  --> src/main.rs:98:34
   |
92 |         println!("BNBUSD={:#?}", mut_ei.get_symbol("BNBUSD"));
   |                                  ------ first mutable borrow occurs here
...
98 |         println!("BNBUSD={:#?}", mut_ei.get_symbol("BNBUSD"));
   |                                  ^^^^^^
   |                                  |
   |                                  second mutable borrow occurs here
   |                                  first borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
error: could not compile `expr-vec-hashmap`
```

When `main` is this:
```
fn main() {
    let mut mut_ei = ExchangeInfo::new();
    println!("mut_ei={:#?}", mut_ei);
    mut_ei.add_symbol(Symbol::new("BTCUSD".to_string()));
    mut_ei.add_symbol(Symbol::new("BNBUSD".to_string()));
    println!("mut_ei={:#?}", mut_ei);

    if false {
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
        println!("BNBUSD={:#?}", mut_ei.get_symbol("BNBUSD"));
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
```

The **right** what is to have explicilty call `create_symbol_hash_map` and
then used the returned immutable `ei`:
```
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
```

And it now compiles and runs and we can use `ei.get_symbol_via_hm` multiple times!
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/expr-vec-hashmap`
mut_ei=ExchangeInfo {
    server_time: 1618870280505,
    symbols: [],
    symbols_hm: {},
}
mut_ei=ExchangeInfo {
    server_time: 1618870280505,
    symbols: [
        Symbol {
            name: "BTCUSD",
        },
        Symbol {
            name: "BNBUSD",
        },
    ],
    symbols_hm: {},
}
BTCUSD=Some(
    Symbol {
        name: "BTCUSD",
    },
)
BNBUSD=Some(
    Symbol {
        name: "BNBUSD",
    },
)
XYZUSD=None
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
