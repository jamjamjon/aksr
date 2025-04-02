# aksr

A Rust derive macro designed to simplify struct management by automatically generating getter and setter methods for both named and tuple structs. The macro supports customizations such as method prefixes, aliases, and incrementable fields, enhancing code readability and reducing boilerplate.

## Installation
Add `aksr` to your `Cargo.toml` dependencies:
```toml
[dependencies]
aksr = "0.0.1"
```

## Example
This example demonstrates the use of `aksr` with a named struct. 

```rust
use aksr::Builder;

#[derive(Builder, Debug, Default)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    #[args(
        aka = "attributes",
        set_pre = "set",
        inc = true,
        getter = false
    )]
    attrs: Vec<String>,
}

fn main() {
    let rect = Rect::default()
        .with_x(0.0)
        .with_y(0.0)
        .with_w(10.0)
        .with_h(5.0)
        .set_attributes(&["A", "X", "Z"])
        .set_attributes_inc(&["O"])
        .set_attributes_inc(&["P"]);

    println!("rect: {:?}", rect);
    println!("x: {}", rect.x());
    println!("y: {}", rect.y());
    println!("w: {}", rect.w());
    println!("h: {}", rect.h());
    println!("attrs: {:?}", rect.attrs);
    // println!("attrs: {:?}", rect.attrs()); // Method `attrs` is not generated
}

```


## Code Expanded

<details>
<summary>Click to view the expanded code for the struct above</summary>



```rust

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    #[args(aka = "attributes", set_pre = "set", inc = true, getter = false)]
    attrs: Vec<String>,
}
impl Rect {
    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn with_y(mut self, x: f32) -> Self {
        self.y = x;
        self
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn with_w(mut self, x: f32) -> Self {
        self.w = x;
        self
    }
    pub fn w(&self) -> f32 {
        self.w
    }
    pub fn with_h(mut self, x: f32) -> Self {
        self.h = x;
        self
    }
    pub fn h(&self) -> f32 {
        self.h
    }
    pub fn set_attributes(mut self, x: &[&str]) -> Self {
        self.attrs = x.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn set_attributes_inc(mut self, x: &[&str]) -> Self {
        if self.attrs.is_empty() {
            self.attrs = x.iter().map(|s| s.to_string()).collect();
        } else {
            let mut x = x.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            self.attrs.append(&mut x);
        }
        self
    }
}


```


</details>



## License
This project is licensed under [LICENSE](LICENSE).



