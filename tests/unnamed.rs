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

#[test]
fn into_and_take_tuple_fields() {
    // Field 5 is Vec<String>
    let ent = Entity::default().with_5(&["s1", "s2"]);
    let v = ent.into_nth_5();
    assert_eq!(v, vec!["s1", "s2"]);

    let mut ent2 = Entity::default().with_5(&["a", "b", "c"]);
    let v2 = ent2.take_nth_5();
    assert_eq!(v2, vec!["a", "b", "c"]);
    assert!(ent2.nth_5().is_empty());

    // Field 8 is Option<String>
    let ent3 = Entity::default().with_8("k");
    let s = ent3.into_nth_8();
    assert_eq!(s, Some("k".to_string()));

    let mut ent4 = Entity::default().with_8("z");
    let s2 = ent4.take_nth_8();
    assert_eq!(s2, Some("z".to_string()));
    assert_eq!(ent4.nth_8(), None);
}

#[test]
fn test_into_and_take_comprehensive_tuple() {
    // Test all tuple field types with into_* and take_*
    let ent = Entity::default()
        .with_1("string1")
        .with_5(&["vec1", "vec2"])
        .with_7(100)
        .with_8("opt_string")
        .with_12(&["opt_vec1", "opt_vec2"]);

    // Test into_nth_1 (String)
    let s = ent.into_nth_1();
    assert_eq!(s, "string1");

    // Test into_nth_5 (Vec<String>)
    let ent2 = Entity::default().with_5(&["a", "b"]);
    let v = ent2.into_nth_5();
    assert_eq!(v, vec!["a", "b"]);

    // Test take_nth_5
    let mut ent3 = Entity::default().with_5(&["x", "y"]);
    let v2 = ent3.take_nth_5();
    assert_eq!(v2, vec!["x", "y"]);
    assert!(ent3.nth_5().is_empty());

    // Test into_nth_8 (Option<String>)
    let ent4 = Entity::default().with_8("test");
    let opt = ent4.into_nth_8();
    assert_eq!(opt, Some("test".to_string()));

    // Test take_nth_8
    let mut ent5 = Entity::default().with_8("take");
    let opt2 = ent5.take_nth_8();
    assert_eq!(opt2, Some("take".to_string()));
    assert_eq!(ent5.nth_8(), None);

    // Test into_nth_12 (Option<Vec<String>>)
    let ent6 = Entity::default().with_12(&["opt1", "opt2"]);
    let opt_vec = ent6.into_nth_12();
    assert_eq!(opt_vec, Some(vec!["opt1".to_string(), "opt2".to_string()]));

    // Test take_nth_12
    let mut ent7 = Entity::default().with_12(&["take1", "take2"]);
    let opt_vec2 = ent7.take_nth_12();
    assert_eq!(
        opt_vec2,
        Some(vec!["take1".to_string(), "take2".to_string()])
    );
    assert_eq!(ent7.nth_12(), None);
}

#[test]
fn test_tuple_into_consumes() {
    // Verify into_* consumes self for tuple structs
    let ent = Entity::default().with_1("test");
    let _s = ent.into_nth_1();
    // ent is now consumed
}

#[test]
fn test_tuple_take_preserves() {
    // Verify take_* preserves self for tuple structs
    let mut ent = Entity::default().with_1("first").with_5(&["a"]);

    let s = ent.take_nth_1();
    assert_eq!(s, "first");
    assert_eq!(ent.nth_1(), ""); // Can still use ent

    let v = ent.take_nth_5();
    assert_eq!(v, vec!["a"]);
    assert!(ent.nth_5().is_empty()); // Can still use ent

    // Can still set new values
    let _ent2 = ent.with_1("new");
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
