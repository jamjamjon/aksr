use aksr::Builder;

#[derive(Builder, Debug, Default)]
struct Rect {
    #[args(getter = false)]
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    #[args(alias = "attributes", setter_prefix = "set", inc = true)]
    attrs: Vec<String>,
}

fn main() {
    let rect = Rect::default()
        .with_x(0.)
        .with_y(0.)
        .with_w(10.)
        .with_h(5.)
        .set_attributes(&["A", "X", "Z"])
        .set_attributes_inc(&["O"])
        .set_attributes_inc(&["P"]);

    println!("rect: {:?}", rect);
    println!("x: {}", rect.x);
    println!("y: {}", rect.y());
    println!("w: {}", rect.w());
    println!("h: {}", rect.h());
    println!("attrs: {:?}", rect.attributes());
    // println!("x: {}", rect.x());  // no method named `x()`, getter disabled
    // println!("attrs: {:?}", rect.attrs()); // no method named `attrs`
}
