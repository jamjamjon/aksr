use aksr::Builder;

#[derive(Builder, Default, Debug)]
struct Color<'a>(
    u8,
    u8,
    u8,
    #[args(aka = "alpha")] f32,
    #[args(inc = true, get_pre = "get", set_pre = "set")] Vec<&'a str>,
);

fn main() {
    let color = Color::default()
        .with_0(255)
        .with_1(255)
        .with_2(0)
        .with_alpha(0.8)
        .set_4(&["A", "B", "C"])
        .set_4_inc(&["D", "E"]);

    println!(
        "RGBA: ({}, {}, {}, {}, {:?})",
        color.nth_0(),
        color.nth_1(),
        color.nth_2(),
        color.alpha(),
        color.get_4(),
    );
}
