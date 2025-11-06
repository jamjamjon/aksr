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
- 🔄 **Ownership methods** - `into_*()` and `take_*()` for efficient value extraction
- ⚡ **Zero-cost abstractions** following Rust best practices


## Installation

Add `aksr` to your `Cargo.toml`:
```toml
[dependencies]
aksr = "0.0.5"
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
|-----------|-------------|--------|-------------------|
| `skip` | Skip both getter and setter | `true`, `false` | `#[args(skip = false)]` |
| `alias` | Field alias for setter/getter | String | `#[args(alias = "field_name")]` |
| `setter` | Control setter generation | `true`, `false` | `#[args(setter = true)]` |
| `getter` | Control getter generation | `true`, `false` | `#[args(getter = true)]` |
| `allow` | Whitelist specific features | `setter`, `getter`, `extend`, `skip` | `#[args(allow(xxx, xxx))]` |
| `except` | Blacklist specific features | `setter`, `getter`, `extend`, `skip`, `into` | `#[args(except(xxx, xxx))]` |
| `visibility` | Control both getter/setter visibility | `"pub"`, `"private"`, `"pub(crate)"`, `"pub(self)"`, `"pub(super)"`, `"pub(in path)"` | `#[args(visibility = "pub(crate)")]` |
| `setter_visibility` | Control setter visibility (overrides `visibility` if present) | same as above | `#[args(setter_visibility = "pub")]` |
| `getter_visibility` | Control getter visibility (overrides `visibility` if present) | same as above | `#[args(getter_visibility = "pub")]` |
| `setter_prefix` | Custom setter prefix | String | `#[args(setter_prefix = "with")]` |
| `getter_prefix` | Custom getter prefix (named / tuple) | String | `#[args(getter_prefix = "field_name")]` / `#[args(getter_prefix = "nth")]` |
| `inline` | Control inline for both getter/setter | `true`, `false`, `"always"` | `#[args(inline = true)]` |
| `getter_inline` | Control getter inline attribute | `true`, `false`, `"always"` | `#[args(getter_inline = "always")]` |
| `setter_inline` | Control setter inline attribute | `true`, `false`, `"always"` | `#[args(setter_inline = true)]` |
| `extend` | Enable extend methods for Vec | `true`, `false` | `#[args(extend = false)]` |
| `into_prefix` | Custom prefix for `into_*()` methods | String (default: `"into"`) | `#[args(into_prefix = "extract")]` |
| `except(into)` | Disable `into_*()` method generation | - | `#[args(except(into))]` |


## Ownership Methods: `into_*()` and `take_*()`

`aksr` automatically generates ownership methods for fields that own their data, allowing efficient value extraction without cloning.

### `into_*()` Methods

**Generated for:**
- ✅ Owned types: `Vec<T>`, `String`, `HashMap<K, V>`, `HashSet<T>`, `BTreeMap<K, V>`, `BTreeSet<T>`, etc.
- ✅ Collections: `VecDeque<T>`, `BinaryHeap<T>`
- ✅ Smart pointers: `Box<T>`, `Rc<T>`, `Arc<T>`, `Weak<T>`
- ✅ Wrappers: `RefCell<T>`, `Mutex<T>`, `RwLock<T>`, `Cow<'a, T>`, `OsString`, `PathBuf`
- ✅ `Option<T>` (any inner type)
- ✅ Custom owned types (non-primitive)
- ✅ Arrays and tuples
- ✅ Reference types (moves the reference itself)

**NOT generated for:**
- ❌ Primitive/Copy types: `usize`, `bool`, `char`, `i32`, `u8`, `f64`, etc. (getters already return by value)
- ❌ Fields with `#[args(except(into))]` attribute

**Behavior:**
- Consumes `self` and moves the field value out
- Zero-cost: no cloning, just ownership transfer
- Follows Rust naming convention (like `into_iter()`, `into_inner()`)

**Example:**
```rust
#[derive(Builder, Default)]
struct Config {
    items: Vec<String>,
    name: String,
    #[args(except(into))]
    metadata: String,  // No into_metadata() generated
}

let config = Config::default()
    .with_items(&["a", "b"])
    .with_name("test");

let items = config.into_items();  // Moves Vec<String> out
// config is now consumed and cannot be used
```

### `take_*()` Methods

**Generated for:**
- ✅ `Option<T>` - Uses `Option::take()`, leaves `None` (no `Default` required)
- ✅ Standard collections (require `Default`):
  - `Vec<T>`, `String`, `HashMap<K, V>`, `HashSet<T>`
  - `BTreeMap<K, V>`, `BTreeSet<T>`, `VecDeque<T>`, `BinaryHeap<T>`
- ✅ Smart pointers: `Box<T>`, `Rc<T>`, `Arc<T>`, `Weak<T>`
- ✅ Wrappers: `RefCell<T>`, `Mutex<T>`, `RwLock<T>`, `Cow<'a, T>`, `OsString`, `PathBuf`

**NOT generated for:**
- ❌ Primitive/Copy types
- ❌ Custom types (unless they implement `Default`)
- ❌ Types not in the whitelist (to avoid forcing `Default` constraints)

**Behavior:**
- Takes the field value and replaces it with `Default::default()`
- Does NOT consume `self` - you can continue using the struct
- For `Option<T>`, leaves `None` instead of requiring `Default`

**Example:**
```rust
#[derive(Builder, Default)]
struct Data {
    items: Vec<String>,
    opt_value: Option<String>,
}

let mut data = Data::default()
    .with_items(&["a", "b"])
    .with_opt_value("test");

let items = data.take_items();  // Moves Vec, leaves empty Vec
assert!(data.items().is_empty());  // Can still use data

let opt = data.take_opt_value();  // Moves Option, leaves None
assert_eq!(data.opt_value(), None);  // Can still use data
```

### Customizing `into_*()` Prefix

If `into_*()` conflicts with your custom methods, you can change the prefix:

```rust
#[derive(Builder, Default)]
struct Example {
    #[args(into_prefix = "extract")]
    items: Vec<String>,  // Generates extract_items() instead of into_items()
}
```



## License
This project is licensed under [LICENSE](LICENSE).



