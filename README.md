# to\_string

See the [Documentation](https://docs.rs/to_string).

Pairs well with the [colored](https://github.com/mackwic/colored) crate.

```rust
use to_string::*;
assert_eq!("0b1111000", 120.to_bin());
assert_eq!("0xcafe", 51966.to_hexa());
assert_eq!("0xdeadbef0", (-559038736).to_hexa());
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
