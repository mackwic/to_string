# to\_string

See the [Documentation](https://docs.rs/to_string).

Pairs well with the [colored](https://github.com/mackwic/colored) crate.

```rust
use to_string::*;
assert_eq!("0b1111000", 120.to_bin());
assert_eq!("0xcafe", 51966.to_hexa());
assert_eq!("0xdeadbeef", (-559038737).to_hexa());
assert_eq!("0o1747", 999.to_octal_string());
assert_eq!("4.21e1", (42.1).to_exp());
```

Provided methods:
- to\_bin() / to\_binary\_string()
- to\_hexa() / to\_hexa\_string()
- to\_octal() / to\_octal\_string()
- to\_exp() / to\_exp\_string()
- to\_pointer() / to\_pointer\_string()
- to\_debug() / to\_debug\_string()

# How to use

Add this in your Cargo.toml:

```toml
[dependencies]
to_string = "0.1"
```
and add this to your lib.rs or main.rs:

```rust
    extern crate to_string;
    use to_string::*;
```

