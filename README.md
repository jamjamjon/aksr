# aksr

A Rust derive macro that automatically generates **Builder Lite** pattern methods for structs, supporting both named and tuple structs with extensive customization options.


## Builder Lite Pattern

`aksr` implements the [Builder Lite](https://matklad.github.io/2022/05/29/builder-lite.html) pattern, where the struct itself acts as the builder. Unlike traditional builders that require a separate builder object, Builder Lite is an incremental, zero-cost evolution from the `new()` or `Default::default()` method.


**Requirements:**
- Struct must implement `Default` trait, OR
- Struct must have a `new()` method

This allows you to use the builder pattern without creating a separate builder type, making it especially useful for rapidly evolving application code.


## Features

- 🚀 **Builder Lite pattern** - Zero-cost, incremental builder evolution
- 🎯 **Field aliases** for more intuitive API
- 📦 **Vector extension** methods for incremental updates  
- 🔧 **Flexible customization** via attributes
- 💡 **String optimization** with `&str` → `String` and `&[String]` support
- 🎁 **Smart Option handling** for `Vec<T>` with empty check
- ⚡ **Zero-cost abstractions** following Rust best practices


## Installation

Add `aksr` to your `Cargo.toml`:
```toml
[dependencies]
aksr = "0.0.4"
```


## Examples

Run the examples to see all features in action:

```bash
cargo run --example rect        # Named struct
cargo run --example color       # Tuple struct
```

To see the generated code:
```bash
cargo install cargo-expand      # Install expand tool
cargo expand --example rect     # View generated code
```

- [`examples/rect.rs`](examples/rect.rs) - Named struct with all features
- [`examples/color.rs`](examples/color.rs) - Tuple struct with all features


## Supported Attributes

| Attribute | Description | Values | Example |
|-----------|-------------|--------|---------|
| `alias` | Field alias for setter/getter | String | `#[args(alias = "width")]` |
| `extend` | Enable extend methods for Vec | `true`, `false` | `#[args(extend = true)]` |
| `setter` | Control setter generation | `true`, `false` | `#[args(setter = false)]` |
| `getter` | Control getter generation | `true`, `false` | `#[args(getter = false)]` |
| `setter_prefix` | Custom setter prefix | String | `#[args(setter_prefix = "set")]` |
| `getter_prefix` | Custom getter prefix | String | `#[args(getter_prefix = "get")]` |
| `getter_visibility` | Control getter visibility | `"pub"`, `"private"`, `"pub(crate)"`, `"pub(self)"`, `"pub(super)"`, `"pub(in path)"` | `#[args(getter_visibility = "private")]` |
| `setter_visibility` | Control setter visibility | `"pub"`, `"private"`, `"pub(crate)"`, `"pub(self)"`, `"pub(super)"`, `"pub(in path)"` | `#[args(setter_visibility = "pub(crate)")]` |
| `skip` | Skip both getter and setter | `true`, `false` | `#[args(skip)]` |
| `allow` | Whitelist specific features | `setter`, `getter`, `extend`, `skip` | `#[args(allow(setter, extend))]` |
| `except` | Blacklist specific features | `setter`, `getter`, `extend`, `skip` | `#[args(except(getter))]` |



## License
This project is licensed under [LICENSE](LICENSE).



