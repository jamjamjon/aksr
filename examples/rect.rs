use aksr::Builder;

#[derive(Builder, Debug, Default)]
struct Rect {
    // Generates nothing - neither getter nor setter
    // Useful for internal fields that shouldn't be exposed in the API
    #[args(skip)]
    _unused: bool,

    // Generates: with_x(v: f32) -> Self
    // No getter generated - useful for write-only internal state
    #[args(setter = true, getter = false)]
    x: f32,

    // Generates: y() -> f32
    // No setter generated - useful for read-only derived fields
    #[args(allow(getter), except(setter))]
    y: f32,

    // Generates: width() -> f32, with_width(v: f32) -> Self
    // Field internally named 'w' but publicly accessed as 'width'
    #[args(alias = "width")]
    w: f32,

    // Generates: height() -> f32, set_height(v: f32) -> Self
    // Field internally named 'h' but publicly accessed as 'height'
    // Custom setter prefix 'set' instead of default 'with'
    #[args(alias = "height", setter_prefix = "set")]
    h: f32,

    // Field internally named 'attributes' but publicly accessed as 'tags'
    // Generates 5 methods:
    //   - tags() -> &[String]                           (getter, returns slice)
    //   - with_tags(v: &[&str]) -> Self                 (setter, accepts string literals)
    //   - with_tags_extend(v: &[&str]) -> Self           (append, appends string literals)
    //   - with_tags_owned(v: &[String]) -> Self          (setter, accepts Vec<String>)
    //   - with_tags_extend_owned(v: &[String]) -> Self   (append, appends Vec<String>)
    #[args(alias = "tags", extend = true)]
    attributes: Vec<String>,

    // Field internally named 'description' but publicly accessed as 'desc'
    // Generates: desc() -> Option<&str>, with_desc(v: &str) -> Self
    // Getter returns Option<&str> (smart conversion from String)
    // Private getter, public setter
    #[args(
        alias = "desc",
        getter_visibility = "private",
        setter_visibility = "pub(crate)"
    )]
    description: Option<String>,
}

fn main() {
    // Example 1: Using string slice literals (&[&str])
    // Demonstrates basic usage with &[&str] for Vec<String>
    let rect1 = Rect::default()
        .with_x(1.) // setter exists
        // .with_y(1.)    // ERROR: no setter (disabled by except(setter))
        .with_width(800.) // alias for 'w'
        .set_height(800.) // alias for 'h' with custom 'set' prefix
        .with_tags(&["rust", "derive", "macro"]) // replace tags
        .with_tags_extend(&["builder", "pattern"]) // append more tags
        .with_desc("A comprehensive example."); // alias for 'description'

    println!("Example 1: Using &[&str]");
    println!("{rect1:#?}");
    println!(
        "y: {}\nwidth: {}\nheight: {}\ntags: {:?}\ndesc: {:?}\n",
        rect1.y(),
        rect1.width(),
        rect1.height(),
        rect1.tags(),
        rect1.desc()
    );

    // Example 2: Using Vec<String> (owned methods)
    let existing = vec!["web".to_string(), "api".to_string()];
    let more = vec!["backend".to_string()];

    let rect = Rect::default()
        .with_x(2.)
        .with_width(1024.)
        .set_height(768.)
        .with_tags_owned(&existing) // setter for Vec<String>
        .with_tags_extend_owned(&more); // append Vec<String>

    println!("Example 2: Using Vec<String>");
    println!("{rect:#?}");
    println!(
        "y: {}\nwidth: {}\nheight: {}\ntags: {:?}\ndesc: {:?}",
        rect.y(),
        rect.width(),
        rect.height(),
        rect.tags(),
        rect.desc()
    );
}
