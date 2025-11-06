use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, RwLock};

use aksr::Builder;

#[derive(Default, Debug, PartialEq)]
pub struct X {
    pub a: u8,
    pub b: f32,
}

#[derive(Debug, PartialEq)]
pub enum Y {
    Unit,
    Tuple(u8, f32),
    Struct { x: u8, y: f32 },
}

#[derive(Builder, Debug)]
pub struct Entity<'a, A: std::fmt::Debug, B> {
    // primitive
    unit: (),
    #[args(skip = false)]
    char: char,
    #[args(allow(getter, setter))]
    bool: bool,
    #[args(alias = "float64")]
    f64: f64,
    f32: f32,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
    isize: isize,
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    usize: usize,

    // str & string
    str: &'a str,
    str_static: &'static str,
    string: String,

    // array & tuple
    tuple: (u8, i8),
    array: [u8; 4],

    // vec
    vec_i8: Vec<i8>,
    vec_str: Vec<&'a str>,
    #[args(extend = true)]
    vec_string: Vec<String>,

    // Testing setter control
    #[args(setter = false)]
    no_setter: u8,

    // Testing getter control
    #[args(getter = false)]
    no_getter: u16,

    // Testing setter_prefix
    #[args(setter_prefix = "set")]
    custom_setter: u32,

    // Testing getter_prefix
    #[args(getter_prefix = "get")]
    custom_getter: u64,

    // Testing getter_visibility
    #[args(getter_visibility = "private")]
    private_getter: i32,

    // Testing setter_visibility
    #[args(setter_visibility = "private")]
    private_setter: i64,

    // Testing except
    #[args(except(setter))]
    no_setter_field: f32,

    // Testing combination: alias + extend
    #[args(alias = "tags", extend = true)]
    keywords: Vec<String>,

    // Testing combination: alias + setter_prefix
    #[args(alias = "width", setter_prefix = "set")]
    w: i32,

    // Testing combination: alias + getter_prefix
    #[args(alias = "height", getter_prefix = "get")]
    h: i32,

    // Testing combination: alias + visibility
    #[args(
        alias = "desc",
        getter_visibility = "private",
        setter_visibility = "private"
    )]
    description: String,

    // Testing combination: allow + except together
    #[args(allow(getter), except(setter))]
    readonly: bool,

    // Testing combination: setter + getter control
    #[args(setter = true, getter = false)]
    write_only: i16,

    vec_vec_string: Vec<Vec<String>>,

    // collections: vec, hashmap, hashset. btreemap, btreeset
    hashmap: HashMap<&'a str, usize>,
    hashset: HashSet<u8>,
    btreemap: BTreeMap<String, i32>,
    btreeset: BTreeSet<u32>,
    vec_deque: VecDeque<String>,
    binary_heap: BinaryHeap<u8>,

    // slice
    slice_str: &'a [&'a str],
    slice_usize: &'a [usize],
    slice_f32: &'a [f32],

    // others
    x: X,
    y: Y,

    // smart pointer
    box_u8: Box<u8>,
    rc_string: Rc<String>,
    weak_rc_string: Weak<String>,
    arc_string: Arc<String>,
    refcell_u8: RefCell<u8>,
    arc_mutex_u8: Arc<Mutex<u8>>,
    arc_rwlock_string: Arc<RwLock<String>>,
    cow_str: Cow<'a, str>,

    // generic & trait
    a: A,
    b: B,

    // Option
    opt_u8: Option<u8>,
    opt_tuple: Option<(u8, i8)>,
    opt_array: Option<[u8; 1]>,
    opt_box_u8: Option<Box<u8>>,
    opt_str: Option<&'a str>,
    opt_string: Option<String>,
    opt_vec_str: Option<Vec<&'a str>>,
    opt_vec_string: Option<Vec<String>>,
    opt_vec_vec_string: Option<Vec<Vec<String>>>,
    opt_opt_usize: Option<Option<usize>>,

    // Reults
    result: Result<u8, String>,

    // phantom data for unused lifetime
    _marker: PhantomData<&'a ()>,
}

impl<A: Default + std::fmt::Debug, B: Default> Default for Entity<'_, A, B> {
    fn default() -> Self {
        Entity {
            unit: (),
            char: '\0',
            bool: false,
            f64: 0.0,
            f32: 0.0,
            i8: 0,
            i16: 0,
            i32: 0,
            i64: 0,
            i128: 0,
            isize: 0,
            u8: 0,
            u16: 0,
            u32: 0,
            u64: 0,
            u128: 0,
            usize: 0,
            str: "",
            str_static: "",
            string: String::new(),
            tuple: (0, 0),
            array: [0; 4],
            vec_i8: Vec::new(),
            vec_str: Vec::new(),
            vec_string: Vec::new(),
            no_setter: 0,
            no_getter: 0,
            custom_setter: 0,
            custom_getter: 0,
            private_getter: 0,
            private_setter: 0,
            no_setter_field: 0.0,
            keywords: Vec::new(),
            w: 0,
            h: 0,
            description: String::new(),
            readonly: false,
            write_only: 0,
            vec_vec_string: Vec::new(),
            hashmap: HashMap::new(),
            hashset: HashSet::new(),
            btreemap: BTreeMap::new(),
            btreeset: BTreeSet::new(),
            vec_deque: VecDeque::new(),
            binary_heap: BinaryHeap::new(),
            slice_str: &[],
            slice_usize: &[],
            slice_f32: &[],
            x: X::default(),
            y: Y::Unit,
            box_u8: Box::new(0),
            rc_string: Rc::new(String::new()),
            weak_rc_string: Rc::downgrade(&Rc::new(String::new())),
            arc_string: Arc::new(String::new()),
            refcell_u8: RefCell::new(0),
            arc_mutex_u8: Arc::new(Mutex::new(0)),
            arc_rwlock_string: Arc::new(RwLock::new(String::new())),
            cow_str: Cow::Borrowed(""),
            a: A::default(),
            b: B::default(),
            opt_u8: None,
            opt_tuple: None,
            opt_array: None,
            opt_box_u8: None,
            opt_str: None,
            opt_vec_str: None,
            opt_string: None,
            opt_vec_string: None,
            opt_vec_vec_string: None,
            opt_opt_usize: None,
            result: Ok(0),
            _marker: PhantomData,
        }
    }
}

#[test]
fn all() {
    let entity = Entity::<u8, String>::default()
        .with_unit(())
        .with_char('c')
        .with_bool(true)
        .with_float64(64.)
        .with_f32(32.)
        .with_i8(8)
        .with_i16(16)
        .with_i32(32)
        .with_i64(64)
        .with_i128(128)
        .with_isize(-1)
        .with_u8(8)
        .with_u16(16)
        .with_u32(32)
        .with_u64(64)
        .with_u128(128)
        .with_usize(0)
        .with_str("str")
        .with_str_static("static_str")
        .with_string("string")
        .with_tuple((1, -1))
        .with_array([1, 2, 3, 4])
        .with_vec_i8(&[1, 2, 3])
        .with_vec_str(&["str1", "str2"])
        .with_vec_string(&["str1", "str2"])
        .with_vec_string_extend(&["str3", "str4"])
        // Test custom_setter (setter_prefix = "set")
        .set_custom_setter(999)
        // Test except(setter) - no setter should exist
        // .set_no_setter_field(1.0) // This should fail - no setter
        // Test combination: alias + extend
        .with_tags(&["rust", "macro"])
        .with_tags_extend(&["builder", "derive"])
        // Test combination: alias + setter_prefix
        .set_width(1920)
        // Test combination: alias + getter_prefix (needs direct access since getter has prefix)
        // h will be accessed via get_height() with prefix "get"
        // Test combination: alias + visibility (private)
        .with_desc("private")
        // Test combination: allow + except
        // .set_readonly(true) // No setter due to except
        // Test combination: setter + getter control
        .with_write_only(256)
        .with_vec_vec_string(&[vec!["inner1".to_string(), "inner2".to_string()]])
        .with_hashmap(HashMap::from([("k", 1)]))
        .with_hashset(HashSet::from([1, 2, 3, 1]))
        .with_btreemap(BTreeMap::from([("k".to_string(), 1)]))
        .with_btreeset(BTreeSet::from([1, 2, 3, 1]))
        .with_vec_deque(VecDeque::from(["element".to_string()]))
        .with_binary_heap(BinaryHeap::from([1, 6, 3, 2, 4]))
        .with_slice_str(&["slice1", "slice2"])
        .with_slice_usize(&[1, 2, 3])
        .with_slice_f32(&[1.0, 2.0, 3.0])
        .with_x(X { a: 5, b: 5. })
        .with_y(Y::Tuple(7, 7.))
        .with_box_u8(Box::new(1))
        .with_rc_string(Rc::new("Rc_String".to_string()))
        .with_weak_rc_string(Weak::new())
        .with_arc_string(Arc::new("Arc_String".to_string()))
        .with_refcell_u8(RefCell::new(1))
        .with_arc_mutex_u8(Arc::new(Mutex::new(1)))
        .with_arc_rwlock_string(Arc::new(RwLock::new("RwLock_String".to_string())))
        .with_cow_str(Cow::Borrowed("borrowed_cow"))
        .with_a(89)
        .with_b(String::from("B"))
        .with_opt_u8(1)
        .with_opt_tuple((1, -1))
        .with_opt_array([1])
        .with_opt_box_u8(Box::new(1))
        .with_opt_str("optional_str")
        .with_opt_vec_str(&["opt_str1", "opt_str2"])
        .with_opt_string("optional_string")
        .with_opt_vec_string(&["optional"])
        .with_opt_vec_vec_string(&[vec!["optional".to_string()]])
        .with_opt_opt_usize(Some(2))
        .with_result(Ok(1))
        .with__marker(PhantomData);

    // Validate all fields
    assert_eq!(entity.char, 'c');
    assert_eq!(entity.char(), 'c');
    assert!(entity.bool);
    assert!(entity.bool());
    assert_eq!(entity.f64, 64.0);
    assert_eq!(entity.float64(), 64.0);
    assert_eq!(entity.f32, 32.0);
    assert_eq!(entity.f32(), 32.0);
    assert_eq!(entity.i8, 8);
    assert_eq!(entity.i8(), 8);
    assert_eq!(entity.i16, 16);
    assert_eq!(entity.i16(), 16);
    assert_eq!(entity.i32, 32);
    assert_eq!(entity.i32(), 32);
    assert_eq!(entity.i64, 64);
    assert_eq!(entity.i64(), 64);
    assert_eq!(entity.i128, 128);
    assert_eq!(entity.i128(), 128);
    assert_eq!(entity.isize, -1);
    assert_eq!(entity.isize(), -1);
    assert_eq!(entity.u8, 8);
    assert_eq!(entity.u8(), 8);
    assert_eq!(entity.u16, 16);
    assert_eq!(entity.u16(), 16);
    assert_eq!(entity.u32, 32);
    assert_eq!(entity.u32(), 32);
    assert_eq!(entity.u64, 64);
    assert_eq!(entity.u64(), 64);
    assert_eq!(entity.u128, 128);
    assert_eq!(entity.u128(), 128);
    assert_eq!(entity.usize, 0);
    assert_eq!(entity.usize(), 0);

    assert_eq!(entity.str, "str");
    assert_eq!(entity.str(), "str");
    assert_eq!(entity.str_static, "static_str");
    assert_eq!(entity.str_static(), "static_str");
    assert_eq!(entity.string, "string".to_string());
    assert_eq!(entity.string(), "string");
    assert_eq!(entity.tuple, (1, -1));
    assert_eq!(entity.tuple(), &(1, -1));
    assert_eq!(entity.array, [1, 2, 3, 4]);
    assert_eq!(entity.array(), &[1, 2, 3, 4]);
    assert_eq!(entity.vec_i8, vec![1, 2, 3]);
    assert_eq!(entity.vec_i8(), &[1, 2, 3]);
    assert_eq!(entity.vec_str, vec!["str1", "str2"]);
    assert_eq!(entity.vec_str(), &["str1", "str2"]);
    assert_eq!(entity.vec_string, vec!["str1", "str2", "str3", "str4"]);
    assert_eq!(entity.vec_string(), &["str1", "str2", "str3", "str4"]);

    // Test custom_setter
    assert_eq!(entity.custom_setter, 999);

    // Test custom_getter (getter_prefix = "get")
    assert_eq!(entity.get_custom_getter(), 0);

    // no_setter has no setter (setter = false), but has getter
    // no_getter has no getter (getter = false), but has setter

    // no_setter_field has no setter (except(setter)), but has getter
    assert_eq!(entity.no_setter_field(), 0.0);

    // Test combination: alias + extend
    assert_eq!(entity.keywords, vec!["rust", "macro", "builder", "derive"]);
    assert_eq!(entity.tags(), &["rust", "macro", "builder", "derive"]);

    // Test combination: alias + setter_prefix
    assert_eq!(entity.w, 1920);
    assert_eq!(entity.width(), 1920);

    // Test combination: alias + getter_prefix
    // Note: getter_prefix affects the prefix in getter name generation
    // With alias="height" and getter_prefix="get", the getter is get_height()
    assert_eq!(entity.h, 0);
    assert_eq!(entity.get_height(), 0);

    // Test combination: alias + visibility
    // Private methods are accessible in same module
    assert_eq!(entity.desc(), "private");
    assert_eq!(entity.description, "private");

    // Test combination: allow + except
    // readonly has allow(getter) and except(setter), so it defaults to false
    assert!(!entity.readonly());

    // Test combination: setter + getter control
    assert_eq!(entity.write_only, 256);
    // No getter for write_only

    assert_eq!(
        entity.vec_vec_string,
        vec![vec!["inner1".to_string(), "inner2".to_string()]]
    );
    assert_eq!(
        entity.vec_vec_string(),
        &[vec!["inner1".to_string(), "inner2".to_string()]]
    );
    assert_eq!(entity.hashmap, HashMap::from([("k", 1)]));
    assert_eq!(entity.hashmap(), &HashMap::from([("k", 1)]));
    assert_eq!(entity.hashset, HashSet::from([1, 2, 3]));
    assert_eq!(entity.hashset(), &HashSet::from([1, 2, 3]));
    assert_eq!(entity.btreemap, BTreeMap::from([("k".to_string(), 1)]));
    assert_eq!(entity.btreemap(), &BTreeMap::from([("k".to_string(), 1)]));
    assert_eq!(entity.btreeset, BTreeSet::from([1, 2, 3]));
    assert_eq!(entity.btreeset(), &BTreeSet::from([1, 2, 3]));
    assert_eq!(entity.vec_deque, VecDeque::from(["element".to_string()]));
    assert_eq!(entity.vec_deque(), &VecDeque::from(["element".to_string()]));
    assert_eq!(
        entity.binary_heap.clone().into_sorted_vec(),
        vec![1, 2, 3, 4, 6]
    );
    assert_eq!(
        entity.binary_heap().clone().into_sorted_vec(),
        vec![1, 2, 3, 4, 6]
    );
    assert_eq!(entity.slice_str, &["slice1", "slice2"]);
    assert_eq!(entity.slice_str(), &["slice1", "slice2"]);
    assert_eq!(entity.slice_usize, &[1, 2, 3]);
    assert_eq!(entity.slice_usize(), &[1, 2, 3]);
    assert_eq!(entity.slice_f32, &[1.0, 2.0, 3.0]);
    assert_eq!(entity.slice_f32(), &[1.0, 2.0, 3.0]);
    assert_eq!(entity.x, X { a: 5, b: 5.0 });
    assert_eq!(entity.x(), &X { a: 5, b: 5.0 });
    assert_eq!(entity.y, Y::Tuple(7, 7.0));
    assert_eq!(entity.y(), &Y::Tuple(7, 7.0));
    assert_eq!(*entity.box_u8, 1);
    assert_eq!(entity.box_u8(), &Box::new(1));
    assert_eq!(&*entity.rc_string, "Rc_String");
    assert_eq!(entity.rc_string(), &Rc::new("Rc_String".to_string()));
    assert!(entity.weak_rc_string.upgrade().is_none());
    assert!(entity.weak_rc_string().upgrade().is_none());
    assert_eq!(&*entity.arc_string, "Arc_String");
    assert_eq!(entity.arc_string(), &Arc::new("Arc_String".to_string()));
    assert_eq!(*entity.refcell_u8.borrow(), 1);
    assert_eq!(*entity.refcell_u8().borrow(), 1);
    assert_eq!(*entity.arc_mutex_u8.lock().unwrap(), 1);
    assert_eq!(*entity.arc_mutex_u8().lock().unwrap(), 1);
    assert_eq!(&*entity.arc_rwlock_string.read().unwrap(), "RwLock_String");
    assert_eq!(
        &*entity.arc_rwlock_string().read().unwrap(),
        "RwLock_String"
    );
    assert_eq!(entity.cow_str, Cow::Borrowed("borrowed_cow"));
    assert_eq!(entity.cow_str(), &Cow::Borrowed("borrowed_cow"));
    assert_eq!(entity.a, 89);
    assert_eq!(entity.a(), &89);
    assert_eq!(entity.b, String::from("B"));
    assert_eq!(entity.b(), "B");
    assert_eq!(entity.opt_u8, Some(1));
    assert_eq!(entity.opt_u8(), Some(1));
    assert_eq!(entity.opt_tuple, Some((1, -1)));
    assert_eq!(entity.opt_tuple(), Some(&(1, -1)));
    assert_eq!(entity.opt_array, Some([1]));
    assert_eq!(entity.opt_array(), Some(&[1]));
    assert_eq!(entity.opt_box_u8, Some(Box::new(1)));
    assert_eq!(entity.opt_box_u8(), Some(&Box::new(1)));
    assert_eq!(entity.opt_str, Some("optional_str"));
    assert_eq!(entity.opt_str(), Some("optional_str"));
    assert_eq!(entity.opt_string, Some("optional_string".to_string()));
    assert_eq!(entity.opt_string(), Some("optional_string"));
    assert_eq!(entity.opt_vec_str, Some(vec!["opt_str1", "opt_str2"]));
    assert_eq!(entity.opt_vec_str(), Some(&["opt_str1", "opt_str2"][..]));
    assert_eq!(entity.opt_vec_string, Some(vec!["optional".to_string()]));
    assert_eq!(entity.opt_vec_string(), Some(&["optional".to_string()][..]));
    assert_eq!(
        entity.opt_vec_vec_string,
        Some(vec![vec!["optional".to_string()]])
    );
    assert_eq!(
        entity.opt_vec_vec_string(),
        Some(&[vec!["optional".to_string()]][..])
    );
    assert_eq!(entity.opt_opt_usize, Some(Some(2)));
    assert_eq!(entity.opt_opt_usize(), Some(&Some(2)));
    assert_eq!(entity.result, Ok(1));
    assert_eq!(entity.result(), &Ok(1));
}

#[test]
fn into_and_take_on_owned_fields() {
    // into_vec_string: consume self, move out Vec<String>
    let entity = Entity::<u8, String>::default().with_vec_string(&["a", "b"]);
    let items = entity.into_vec_string();
    assert_eq!(items, vec!["a", "b"]);

    // take_vec_string: move out, leave default (empty Vec)
    let mut e2 = Entity::<u8, String>::default().with_vec_string(&["x", "y"]);
    let taken = e2.take_vec_string();
    assert_eq!(taken, vec!["x", "y"]);
    assert!(e2.vec_string().is_empty());

    // into_string: consume self, move out String
    let e3 = Entity::<u8, String>::default().with_string("hello");
    let s = e3.into_string();
    assert_eq!(s, "hello");

    // take_string: move out, leave default (empty String)
    let mut e4 = Entity::<u8, String>::default().with_string("world");
    let s2 = e4.take_string();
    assert_eq!(s2, "world");
    assert_eq!(e4.string(), "");

    // Option<String>: into_*, take_*
    let e5 = Entity::<u8, String>::default().with_opt_string("opt");
    let opt = e5.into_opt_string();
    assert_eq!(opt, Some("opt".to_string()));

    let mut e6 = Entity::<u8, String>::default().with_opt_string("z");
    let taken_opt = e6.take_opt_string();
    assert_eq!(taken_opt, Some("z".to_string()));
    assert_eq!(e6.opt_string(), None);
}

// Test empty Vec handling
#[test]
fn test_empty_vec_should_not_update() {
    let entity = Entity::<u8, String> {
        vec_string: vec!["a".to_string(), "b".to_string()],
        ..Default::default()
    };

    // Empty slice should not update the field
    let result = entity.with_vec_string(&[]);
    assert_eq!(result.vec_string, vec!["a", "b"]);
}

#[test]
fn test_empty_vec_for_option_should_keep_none() {
    let entity = Entity::<u8, String>::default();

    // Empty slice for Option<Vec<String>> should keep it as None
    let result = entity.with_opt_vec_string(&[]);
    assert_eq!(result.opt_vec_string, None);
}

#[test]
fn test_non_empty_vec_should_update() {
    let entity = Entity::<u8, String>::default();

    // Non-empty Vec<String> should update
    let result = entity.with_vec_string(&["c", "d"]);
    assert_eq!(result.vec_string, vec!["c", "d"]);

    // Non-empty Vec for Option<Vec<String>> should set Some
    let result = result.with_opt_vec_string(&["e", "f"]);
    assert_eq!(
        result.opt_vec_string,
        Some(vec!["e".to_string(), "f".to_string()])
    );
}

// Test nested Option handling
#[test]
fn test_nested_option_with_none() {
    let entity = Entity::<u8, String>::default();

    // Setting to Some(Some(2)) should work
    let result = entity.with_opt_opt_usize(Some(2));
    assert_eq!(result.opt_opt_usize, Some(Some(2)));

    // Setting to None should keep the field unchanged
    let result = result.with_opt_opt_usize(None);
    assert_eq!(result.opt_opt_usize, Some(Some(2)));
}

#[test]
fn test_nested_option_with_some_none() {
    let entity = Entity::<u8, String> {
        opt_opt_usize: Some(None),
        ..Default::default()
    };

    // Passing None should keep Some(None)
    let result = entity.with_opt_opt_usize(None);
    assert_eq!(result.opt_opt_usize, Some(None));

    // Setting to Some(5) should set field to Some(Some(5))
    let result = result.with_opt_opt_usize(Some(5));
    assert_eq!(result.opt_opt_usize, Some(Some(5)));
}

#[test]
fn test_vec_string_owned() {
    let existing = vec!["rust".to_string(), "tokio".to_string()];
    let entity = Entity::<u8, String>::default().with_vec_string_owned(&existing);

    assert_eq!(entity.vec_string, vec!["rust", "tokio"]);
}

#[test]
fn test_vec_string_extend_owned() {
    let initial = vec!["a".to_string(), "b".to_string()];
    let more = vec!["c".to_string(), "d".to_string()];

    let entity = Entity::<u8, String>::default()
        .with_vec_string_owned(&initial)
        .with_vec_string_extend_owned(&more);

    assert_eq!(entity.vec_string, vec!["a", "b", "c", "d"]);
}

#[test]
fn test_option_vec_string_owned() {
    let categories = vec!["web".to_string(), "backend".to_string()];
    let entity = Entity::<u8, String>::default().with_opt_vec_string_owned(&categories);

    assert_eq!(
        entity.opt_vec_string,
        Some(vec!["web".to_string(), "backend".to_string()])
    );
}

#[test]
fn test_empty_owned_vec_should_not_update() {
    let entity = Entity::<u8, String> {
        vec_string: vec!["initial".to_string()],
        ..Default::default()
    };

    let empty: Vec<String> = vec![];
    let result = entity.with_vec_string_owned(&empty);

    assert_eq!(result.vec_string, vec!["initial"]);
}

#[test]
fn test_mixed_borrowed_and_owned() {
    let owned = vec!["rust".to_string(), "tokio".to_string()];
    let owned2 = vec!["async".to_string()];

    let entity = Entity::<u8, String>::default()
        .with_vec_string_owned(&owned)
        .with_vec_string_extend(&["derive"])
        .with_vec_string_extend_owned(&owned2);

    assert_eq!(entity.vec_string, vec!["rust", "tokio", "derive", "async"]);
}

// Test getter for Option<Vec<String>>
#[test]
fn test_option_vec_string_getter() {
    // Test getter with Some value
    let entity = Entity::<u8, String>::default().with_opt_vec_string(&["rust", "go", "python"]);

    assert_eq!(
        entity.opt_vec_string(),
        Some(&["rust".to_string(), "go".to_string(), "python".to_string()][..])
    );

    // Test getter with None (empty slice)
    let entity_none = Entity::<u8, String>::default().with_opt_vec_string(&[]);

    assert_eq!(entity_none.opt_vec_string(), None);
    assert_eq!(entity_none.opt_vec_string, None);
}

#[derive(Builder, Debug, Default)]
struct PrefixTest {
    // default behavior: setter_prefix="with", getter_prefix=""
    default_field: String,
    #[args(setter_prefix = "set")]
    custom_setter: u32,
    #[args(getter_prefix = "get")]
    custom_getter: u64,
    #[args(setter_prefix = "set", getter_prefix = "get")]
    both_custom: i32,
    #[args(setter_prefix = "")]
    empty_setter: String,
    #[args(getter_prefix = "")]
    empty_getter: u64,
    #[args(setter_prefix = "", getter_prefix = "")]
    both_empty: bool,
    #[args(alias = "renamed")]
    alias_default: String,
    #[args(alias = "alias_setter", setter_prefix = "set")]
    alias_custom_setter: u32,
    #[args(alias = "alias_getter", getter_prefix = "get")]
    alias_custom_getter: u64,
    #[args(alias = "full_custom", setter_prefix = "set", getter_prefix = "get")]
    alias_both_custom: i32,
    #[args(alias = "direct_name", getter_prefix = "")]
    alias_empty_getter: String,
}

#[test]
fn test_prefix_combinations() {
    let test = PrefixTest::default()
        .with_default_field("default")
        .set_custom_setter(100)
        .with_custom_getter(200)
        .set_both_custom(300)
        .with_empty_setter("empty_s")
        .with_empty_getter(400)
        .with_both_empty(true)
        .with_renamed("renamed_value")
        .set_alias_setter(500)
        .with_alias_getter(600)
        .set_full_custom(700)
        .with_direct_name("direct");
    assert_eq!(test.default_field(), "default");
    assert_eq!(test.custom_setter(), 100);
    assert_eq!(test.get_custom_getter(), 200);
    assert_eq!(test.get_both_custom(), 300);
    assert_eq!(test.empty_setter(), "empty_s");
    assert_eq!(test.empty_getter(), 400);
    assert!(test.both_empty());
    assert_eq!(test.renamed(), "renamed_value");
    assert_eq!(test.alias_setter(), 500);
    assert_eq!(test.get_alias_getter(), 600);
    assert_eq!(test.get_full_custom(), 700);
    assert_eq!(test.direct_name(), "direct");
}

#[derive(Builder, Debug, Default)]
struct IntoTest {
    // Primitive types should NOT have into_* methods
    usize_field: usize,
    bool_field: bool,

    // Owned types should have into_* methods
    string_field: String,
    vec_field: Vec<u8>,

    // Can disable into_* with except(into)
    #[args(except(into))]
    disabled_into: String,

    // Can customize into prefix
    #[args(into_prefix = "extract")]
    custom_prefix: Vec<String>,
}

#[test]
fn test_primitive_no_into() {
    // Primitive types should NOT have into_* methods
    let test = IntoTest::default()
        .with_usize_field(42)
        .with_bool_field(true);

    assert_eq!(test.usize_field(), 42);
    assert!(test.bool_field());
    // test.into_usize_field() should NOT exist - compile error if uncommented
    // test.into_bool_field() should NOT exist - compile error if uncommented
}

#[test]
fn test_owned_types_have_into() {
    // Owned types should have both getters (returning &T) and into_* methods
    let test = IntoTest::default()
        .with_string_field("hello")
        .with_vec_field(&[1, 2, 3]);

    assert_eq!(test.string_field(), "hello");
    let moved_string = test.into_string_field();
    assert_eq!(moved_string, "hello");
}

#[test]
fn test_vec_into() {
    let test = IntoTest::default().with_vec_field(&[1, 2, 3]);
    assert_eq!(test.vec_field(), &[1, 2, 3]);
    let moved_vec = test.into_vec_field();
    assert_eq!(moved_vec, vec![1, 2, 3]);
}

#[test]
fn test_except_into() {
    // except(into): should have getter but NO into_* method
    let test = IntoTest::default().with_disabled_into("test");
    assert_eq!(test.disabled_into(), "test");
    // test.into_disabled_into() should NOT exist - compile error if uncommented
}

#[test]
fn test_custom_into_prefix() {
    // Custom prefix: should use "extract_" instead of "into_"
    let test = IntoTest::default().with_custom_prefix(&["a", "b"]);
    assert_eq!(test.custom_prefix(), &["a", "b"]);
    let extracted = test.extract_custom_prefix();
    assert_eq!(extracted, vec!["a", "b"]);
    // test.into_custom_prefix() should NOT exist - compile error if uncommented
}

// Comprehensive test struct for into_* and take_* methods
#[derive(Builder, Debug, Default)]
struct ComprehensiveTest {
    // Vec types
    vec_i32: Vec<i32>,
    vec_string: Vec<String>,

    // String
    string: String,

    // Option types
    opt_string: Option<String>,
    opt_vec: Option<Vec<i32>>,
    opt_i32: Option<i32>,

    // Collections
    hashmap: HashMap<String, i32>,
    hashset: HashSet<String>,
    btreemap: BTreeMap<String, i32>,
    btreeset: BTreeSet<String>,

    // Smart pointers
    box_u8: Box<u8>,

    // Arrays and tuples
    array: [i32; 3],
    tuple: (String, i32),
}

#[test]
fn test_into_vec_types() {
    // Test into_vec_i32
    let test = ComprehensiveTest::default().with_vec_i32(&[1, 2, 3, 4, 5]);

    assert_eq!(test.vec_i32(), &[1, 2, 3, 4, 5]);
    let moved = test.into_vec_i32();
    assert_eq!(moved, vec![1, 2, 3, 4, 5]);

    // Test into_vec_string
    let test2 = ComprehensiveTest::default().with_vec_string(&["a", "b", "c"]);

    assert_eq!(test2.vec_string(), &["a", "b", "c"]);
    let moved2 = test2.into_vec_string();
    assert_eq!(moved2, vec!["a", "b", "c"]);
}

#[test]
fn test_take_vec_types() {
    // Test take_vec_i32
    let mut test = ComprehensiveTest::default().with_vec_i32(&[10, 20, 30]);

    assert_eq!(test.vec_i32(), &[10, 20, 30]);
    let taken = test.take_vec_i32();
    assert_eq!(taken, vec![10, 20, 30]);
    assert!(test.vec_i32().is_empty()); // Should be empty after take

    // Test take_vec_string
    let mut test2 = ComprehensiveTest::default().with_vec_string(&["x", "y", "z"]);

    assert_eq!(test2.vec_string(), &["x", "y", "z"]);
    let taken2 = test2.take_vec_string();
    assert_eq!(taken2, vec!["x", "y", "z"]);
    assert!(test2.vec_string().is_empty()); // Should be empty after take
}

#[test]
fn test_into_string() {
    let test = ComprehensiveTest::default().with_string("hello world");

    assert_eq!(test.string(), "hello world");
    let moved = test.into_string();
    assert_eq!(moved, "hello world");
}

#[test]
fn test_take_string() {
    let mut test = ComprehensiveTest::default().with_string("test string");

    assert_eq!(test.string(), "test string");
    let taken = test.take_string();
    assert_eq!(taken, "test string");
    assert_eq!(test.string(), ""); // Should be empty after take
}

#[test]
fn test_into_option_types() {
    // Test into_opt_string with Some
    let test = ComprehensiveTest::default().with_opt_string("some value");

    assert_eq!(test.opt_string(), Some("some value"));
    let moved = test.into_opt_string();
    assert_eq!(moved, Some("some value".to_string()));

    // Test into_opt_string with None
    let test2 = ComprehensiveTest::default();
    assert_eq!(test2.opt_string(), None);
    let moved2 = test2.into_opt_string();
    assert_eq!(moved2, None);

    // Test into_opt_vec with Some
    let test3 = ComprehensiveTest::default().with_opt_vec(&[1, 2, 3]);

    assert_eq!(test3.opt_vec(), Some(&[1, 2, 3][..]));
    let moved3 = test3.into_opt_vec();
    assert_eq!(moved3, Some(vec![1, 2, 3]));

    // Test into_opt_i32 with Some
    let test4 = ComprehensiveTest::default().with_opt_i32(42);

    assert_eq!(test4.opt_i32(), Some(42));
    let moved4 = test4.into_opt_i32();
    assert_eq!(moved4, Some(42));
}

#[test]
fn test_take_option_types() {
    // Test take_opt_string with Some
    let mut test = ComprehensiveTest::default().with_opt_string("take me");

    assert_eq!(test.opt_string(), Some("take me"));
    let taken = test.take_opt_string();
    assert_eq!(taken, Some("take me".to_string()));
    assert_eq!(test.opt_string(), None); // Should be None after take

    // Test take_opt_string with None
    let mut test2 = ComprehensiveTest::default();
    assert_eq!(test2.opt_string(), None);
    let taken2 = test2.take_opt_string();
    assert_eq!(taken2, None);
    assert_eq!(test2.opt_string(), None); // Should still be None

    // Test take_opt_vec with Some
    let mut test3 = ComprehensiveTest::default().with_opt_vec(&[5, 6, 7]);

    assert_eq!(test3.opt_vec(), Some(&[5, 6, 7][..]));
    let taken3 = test3.take_opt_vec();
    assert_eq!(taken3, Some(vec![5, 6, 7]));
    assert_eq!(test3.opt_vec(), None); // Should be None after take

    // Test take_opt_i32 with Some
    let mut test4 = ComprehensiveTest::default().with_opt_i32(100);

    assert_eq!(test4.opt_i32(), Some(100));
    let taken4 = test4.take_opt_i32();
    assert_eq!(taken4, Some(100));
    assert_eq!(test4.opt_i32(), None); // Should be None after take
}

#[test]
fn test_into_collections() {
    // Test into_hashmap
    let mut map = HashMap::new();
    map.insert("key1".to_string(), 1);
    map.insert("key2".to_string(), 2);

    let test = ComprehensiveTest::default().with_hashmap(map.clone());

    assert_eq!(test.hashmap(), &map);
    let moved = test.into_hashmap();
    assert_eq!(moved, map);

    // Test into_hashset
    let set: HashSet<String> = ["a".to_string(), "b".to_string()].iter().cloned().collect();
    let test2 = ComprehensiveTest::default().with_hashset(set.clone());

    assert_eq!(test2.hashset(), &set);
    let moved2 = test2.into_hashset();
    assert_eq!(moved2, set);

    // Test into_btreemap
    let mut bmap = BTreeMap::new();
    bmap.insert("k1".to_string(), 10);
    bmap.insert("k2".to_string(), 20);

    let test3 = ComprehensiveTest::default().with_btreemap(bmap.clone());

    assert_eq!(test3.btreemap(), &bmap);
    let moved3 = test3.into_btreemap();
    assert_eq!(moved3, bmap);

    // Test into_btreeset
    let bset: BTreeSet<String> = ["x".to_string(), "y".to_string()].iter().cloned().collect();
    let test4 = ComprehensiveTest::default().with_btreeset(bset.clone());

    assert_eq!(test4.btreeset(), &bset);
    let moved4 = test4.into_btreeset();
    assert_eq!(moved4, bset);
}

#[test]
fn test_take_collections() {
    // Test take_hashmap
    let mut map = HashMap::new();
    map.insert("k1".to_string(), 1);

    let mut test = ComprehensiveTest::default().with_hashmap(map.clone());

    assert_eq!(test.hashmap(), &map);
    let taken = test.take_hashmap();
    assert_eq!(taken, map);
    assert!(test.hashmap().is_empty()); // Should be empty after take

    // Test take_hashset
    let set: HashSet<String> = ["a".to_string()].iter().cloned().collect();
    let mut test2 = ComprehensiveTest::default().with_hashset(set.clone());

    assert_eq!(test2.hashset(), &set);
    let taken2 = test2.take_hashset();
    assert_eq!(taken2, set);
    assert!(test2.hashset().is_empty()); // Should be empty after take

    // Test take_btreemap
    let mut bmap = BTreeMap::new();
    bmap.insert("k".to_string(), 5);

    let mut test3 = ComprehensiveTest::default().with_btreemap(bmap.clone());

    assert_eq!(test3.btreemap(), &bmap);
    let taken3 = test3.take_btreemap();
    assert_eq!(taken3, bmap);
    assert!(test3.btreemap().is_empty()); // Should be empty after take

    // Test take_btreeset
    let bset: BTreeSet<String> = ["z".to_string()].iter().cloned().collect();
    let mut test4 = ComprehensiveTest::default().with_btreeset(bset.clone());

    assert_eq!(test4.btreeset(), &bset);
    let taken4 = test4.take_btreeset();
    assert_eq!(taken4, bset);
    assert!(test4.btreeset().is_empty()); // Should be empty after take
}

#[test]
fn test_into_smart_pointers() {
    // Test into_box_u8
    let test = ComprehensiveTest::default().with_box_u8(Box::new(42));

    assert_eq!(test.box_u8(), &Box::new(42));
    let moved = test.into_box_u8();
    assert_eq!(*moved, 42);
}

#[test]
fn test_take_smart_pointers() {
    // Test take_box_u8
    let mut test = ComprehensiveTest::default().with_box_u8(Box::new(100));

    assert_eq!(test.box_u8(), &Box::new(100));
    let taken = test.take_box_u8();
    assert_eq!(*taken, 100);
    // After take, should be default (0 in box)
    assert_eq!(**test.box_u8(), 0);
}

#[test]
fn test_into_arrays_and_tuples() {
    // Test into_array
    let test = ComprehensiveTest::default().with_array([1, 2, 3]);

    assert_eq!(test.array(), &[1, 2, 3]);
    let moved = test.into_array();
    assert_eq!(moved, [1, 2, 3]);

    // Test into_tuple
    let test2 = ComprehensiveTest::default().with_tuple(("hello".to_string(), 42));

    assert_eq!(test2.tuple(), &("hello".to_string(), 42));
    let moved2 = test2.into_tuple();
    assert_eq!(moved2, ("hello".to_string(), 42));
}

#[test]
fn test_into_consumes_self() {
    // Verify that into_* consumes self
    let test = ComprehensiveTest::default()
        .with_string("test")
        .with_vec_i32(&[1, 2]);

    let _moved_string = test.into_string();
    // test is now consumed, cannot use it anymore
    // This is verified by compilation - if we try to use test here, it won't compile
}

#[test]
fn test_take_preserves_self() {
    // Verify that take_* preserves self
    let mut test = ComprehensiveTest::default()
        .with_string("test")
        .with_vec_i32(&[1, 2, 3])
        .with_opt_string("optional");

    let moved_string = test.take_string();
    assert_eq!(moved_string, "test");
    assert_eq!(test.string(), ""); // Can still use test

    let moved_vec = test.take_vec_i32();
    assert_eq!(moved_vec, vec![1, 2, 3]);
    assert!(test.vec_i32().is_empty()); // Can still use test

    let moved_opt = test.take_opt_string();
    assert_eq!(moved_opt, Some("optional".to_string()));
    assert_eq!(test.opt_string(), None); // Can still use test

    // Verify we can still set new values
    let _test2 = test.with_string("new value");
}

#[test]
fn test_multiple_take_operations() {
    // Test multiple take operations on the same struct
    let mut test = ComprehensiveTest::default()
        .with_string("first")
        .with_vec_i32(&[1])
        .with_opt_string("second");

    let s1 = test.take_string();
    assert_eq!(s1, "first");

    let v1 = test.take_vec_i32();
    assert_eq!(v1, vec![1]);

    let o1 = test.take_opt_string();
    assert_eq!(o1, Some("second".to_string()));

    // All fields should be in default state
    assert_eq!(test.string(), "");
    assert!(test.vec_i32().is_empty());
    assert_eq!(test.opt_string(), None);
}

#[test]
fn test_into_with_empty_values() {
    // Test into_* with empty collections
    let test = ComprehensiveTest::default(); // All fields are empty/default

    let vec = test.into_vec_i32();
    assert!(vec.is_empty());

    let test2 = ComprehensiveTest::default();
    let string = test2.into_string();
    assert_eq!(string, "");

    let test3 = ComprehensiveTest::default();
    let opt = test3.into_opt_string();
    assert_eq!(opt, None);
}

#[test]
fn test_take_with_empty_values() {
    // Test take_* with empty collections
    let mut test = ComprehensiveTest::default();

    let vec = test.take_vec_i32();
    assert!(vec.is_empty());
    assert!(test.vec_i32().is_empty()); // Still empty after take

    let mut test2 = ComprehensiveTest::default();
    let string = test2.take_string();
    assert_eq!(string, "");
    assert_eq!(test2.string(), ""); // Still empty after take

    let mut test3 = ComprehensiveTest::default();
    let opt = test3.take_opt_string();
    assert_eq!(opt, None);
    assert_eq!(test3.opt_string(), None); // Still None after take
}
