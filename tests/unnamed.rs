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

#[derive(Builder, Debug, Default)]
struct TuplePrefixTest(
    u32,
    #[args(setter_prefix = "set")] u64,
    #[args(getter_prefix = "get")] i32,
    #[args(setter_prefix = "set", getter_prefix = "get")] bool,
    #[args(setter_prefix = "")] String,
    #[args(getter_prefix = "")] u32,
    #[args(setter_prefix = "", getter_prefix = "")] u64,
    #[args(alias = "value")] i32,
    #[args(alias = "name", setter_prefix = "set")] String,
    #[args(alias = "data", getter_prefix = "get")] u32,
    #[args(alias = "custom", setter_prefix = "set", getter_prefix = "get")] bool,
    #[args(alias = "direct", getter_prefix = "")] u64,
    #[args(alias = "alias_empty_setter", setter_prefix = "")] String,
);

#[test]
fn test_tuple_prefix_combinations() {
    let test = TuplePrefixTest::default()
        .with_0(100)
        .set_1(200)
        .with_2(300)
        .set_3(true)
        .with_4("empty_setter")
        .with_5(400)
        .with_6(500)
        .with_value(600)
        .set_name("custom_name")
        .with_data(700)
        .set_custom(false)
        .with_direct(800)
        .with_alias_empty_setter("test");
    assert_eq!(test.nth_0(), 100);
    assert_eq!(test.nth_1(), 200);
    assert_eq!(test.get_2(), 300);
    assert!(test.get_3());
    assert_eq!(test.nth_4(), "empty_setter");
    assert_eq!(test.nth_5(), 400);
    assert_eq!(test.nth_6(), 500);
    assert_eq!(test.value(), 600);
    assert_eq!(test.name(), "custom_name");
    assert_eq!(test.get_data(), 700);
    assert!(!test.get_custom());
    assert_eq!(test.direct(), 800);
    assert_eq!(test.alias_empty_setter(), "test");
}
