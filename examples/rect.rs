use aksr::Builder;

#[derive(Builder, Debug, Default)]
struct Rect {
    #[args(skip)]
    x: f32,
    #[args(allow(getter), except(setter))]
    y: f32,
    #[args(aka = "width")]
    w: f32,
    h: f32,
    #[args(
        aka = "attributes",
        set_pre = "set",
        setter = false,
        getter,
        allow(setter, inc),
        except(getter)
    )]
    attrs: Vec<String>,
}

fn main() {
    let rect = Rect::default()
        // .with_x(0.)     // uncomment this line to see the error
        // .with_y(0.)  // uncomment this line to see the error
        .with_width(10.)
        .with_h(5.)
        .set_attributes(&["A", "X", "Z"])
        .set_attributes_inc(&["O"])
        .set_attributes_inc(&["P"]);

    println!("rect: {:?}", rect);
    println!("x: {}", rect.x);
    // println!("x: {}", rect.x()); // uncomment this line to see the error
    println!("y: {}", rect.y());
    println!("w: {}", rect.width());
    println!("h: {}", rect.h());
    // println!("x: {}", rect.x());  // no method named `x()`, getter disabled
    // println!("attrs: {:?}", rect.attrs()); // no method named `attrs`
}
