use aksr::Builder;

#[derive(Builder, Debug, Default)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    #[args(alias = "attributes", prefix = "add", inc = true, getter = false)]
    attrs: Vec<String>,
}

fn main() {
    let rect = Rect::default()
        .with_x(0.)
        .with_y(0.)
        .with_w(10.)
        .with_h(5.)
        .add_attributes(&["A", "X", "Z"])
        .add_attributes_inc(&["O"])
        .add_attributes_inc(&["P"]);

    println!("rect: {:?}", rect);
    println!("x: {:?}", rect.x());
    println!("y: {:?}", rect.y());
    println!("w: {:?}", rect.w());
    println!("h: {:?}", rect.h());
    println!("attrs: {:?}", rect.attrs);
}
