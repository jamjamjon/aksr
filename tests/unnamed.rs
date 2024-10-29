use aksr::Builder;

#[derive(Builder, Default, Debug, PartialEq)]
pub struct X {
    pub a: u8,
    pub b: f32,
}

#[derive(Builder, Debug, Default)]
struct Entity<'a>(
    usize,
    String,
    &'a str,
    [u8; 2],
    (u8, u16),
    Vec<String>,
    Vec<&'a str>,
    Option<isize>,
    Option<String>,
    Option<&'a str>,
    Option<Vec<isize>>,
    Option<Vec<&'a str>>,        // &[&'a str]
    Option<Vec<String>>,         // &[&str]
    Option<Vec<Vec<String>>>,    // &[Vec<String>]
    Option<Option<Vec<String>>>, // Option<Vec<String>>
);

#[test]
fn all() {
    let entity = Entity::default()
        .with_0(1)
        .with_1("string")
        .with_2("str")
        .with_3([1, 2])
        .with_4((1, 1))
        .with_5(&["string1", "string2"])
        .with_6(&["str1", "str2"])
        .with_7(9)
        .with_8("string")
        .with_9("str")
        .with_10(&[-1, -2])
        .with_11(&["str1", "str2"])
        .with_12(&["string1", "string2"])
        .with_13(&[vec!["string1".to_string(), "string2".to_string()]])
        .with_14(Some(vec!["string1".to_string(), "string2".to_string()]));

    assert_eq!(entity.nth_0(), 1);
    assert_eq!(entity.nth_1(), "string");
    assert_eq!(entity.nth_2(), "str");
    assert_eq!(entity.nth_3(), &[1, 2]);
    assert_eq!(entity.nth_4(), &(1, 1));
    assert_eq!(
        entity.nth_5(),
        &["string1".to_string(), "string2".to_string()]
    );
    assert_eq!(entity.nth_6(), &["str1", "str2"]);
    assert_eq!(entity.nth_7(), Some(9));
    assert_eq!(entity.nth_8(), Some("string"));
    assert_eq!(entity.nth_9(), Some("str"));
    assert_eq!(entity.nth_10(), Some(&[-1, -2][..]));
    assert_eq!(entity.nth_11(), Some(&["str1", "str2"][..]));
    assert_eq!(
        entity.nth_12(),
        Some(&["string1".to_string(), "string2".to_string()][..])
    );
    assert_eq!(
        entity.nth_13(),
        Some(&[vec!["string1".to_string(), "string2".to_string()]][..])
    );
    assert_eq!(
        entity.nth_14(),
        Some(&Some(vec!["string1".to_string(), "string2".to_string()]))
    );
}
