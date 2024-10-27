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
    char: char,
    bool: bool,
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
    #[args(inc = true)]
    vec_string: Vec<String>,
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
    opt_opt_usize: Option<Option<usize>>,

    // Reults
    result: Result<u8, String>,

    // phantom data for unused lifetime
    _marker: PhantomData<&'a ()>,
}

impl<'a, A: Default + std::fmt::Debug, B: Default> Default for Entity<'a, A, B> {
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
            opt_opt_usize: None,
            result: Ok(0),
            _marker: PhantomData,
        }
    }
}

#[test]
fn setters_and_getters() {
    let entity: Entity<'_, u8, String> = Entity::default()
        .with_unit(())
        .with_char('c')
        .with_bool(true)
        .with_f64(64.)
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
        .with_vec_string_inc(&["str3", "str4"])
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
        .with_opt_opt_usize(Some(2))
        .with_result(Ok(1))
        .with__marker(PhantomData);

    // Validate all fields
    assert_eq!(entity.char, 'c');
    assert_eq!(entity.char(), 'c');
    assert!(entity.bool);
    assert!(entity.bool());
    assert_eq!(entity.f64, 64.0);
    assert_eq!(entity.f64(), 64.0);
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
    assert_eq!(entity.opt_opt_usize, Some(Some(2)));
    assert_eq!(entity.opt_opt_usize(), Some(&Some(2)));
    assert_eq!(entity.result, Ok(1));
    assert_eq!(entity.result(), &Ok(1));
}
