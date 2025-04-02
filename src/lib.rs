//! # aksr
//!
//! `aksr` is a Rust derive macro designed to simplify struct management by automatically generating getter and setter methods for both named and tuple structs.
//!
//!
//! ## Example: Named Struct
//!
//! This example demonstrates the use of `aksr` with a named struct, `Rect`. The `attrs` field is set with an alias, a custom setter prefix, and the ability to increment values, while disabling the generation of a getter method for `attrs`.
//!
//! ```rust
//! use aksr::Builder;
//!
//! #[derive(Builder, Debug, Default)]
//! struct Rect {
//!     x: f32,
//!     y: f32,
//!     w: f32,
//!     h: f32,
//!     #[args(
//!         aka = "attributes",
//!         set_pre = "set",
//!         inc = true,
//!         getter = false
//!     )]
//!     attrs: Vec<String>,
//! }
//!
//! let rect = Rect::default()
//!     .with_x(0.0)
//!     .with_y(0.0)
//!     .with_w(10.0)
//!     .with_h(5.0)
//!     .set_attributes(&["A", "X", "Z"])
//!     .set_attributes_inc(&["O"])
//!     .set_attributes_inc(&["P"]);
//!
//! println!("rect: {:?}", rect);
//! println!("x: {}", rect.x());
//! println!("y: {}", rect.y());
//! println!("w: {}", rect.w());
//! println!("h: {}", rect.h());
//! println!("attrs: {:?}", rect.attrs);
//! // println!("attrs: {:?}", rect.attrs()); // Method `attrs` is not generated
//! ```
//!
//! ## Example: Tuple Struct
//!
//! Here, `aksr` is used with a tuple struct, `Color`. The example demonstrates customizing getter and setter prefixes, defining an alias for a specific field, and configuring one field to be incrementable.
//!
//! ```rust
//! use aksr::Builder;
//!
//! #[derive(Builder, Default, Debug)]
//! struct Color<'a>(
//!     u8,
//!     u8,
//!     u8,
//!     #[args(aka = "alpha")] f32,
//!     #[args(inc = true, get_pre = "get", set_pre = "set")] Vec<&'a str>,
//! );
//!
//! let color = Color::default()
//!     .with_0(255)
//!     .with_1(255)
//!     .with_2(0)
//!     .with_alpha(0.8)
//!     .set_4(&["A", "B", "C"])
//!     .set_4_inc(&["D", "E"]);
//!
//! println!(
//!     "RGBA: ({}, {}, {}, {}, {:?})",
//!     color.nth_0(),
//!     color.nth_1(),
//!     color.nth_2(),
//!     color.alpha(),
//!     color.get_4(),
//! );
//! ```
//!

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, GenericArgument, Index, PathArguments,
    Type,
};

mod rules;
use rules::Rules;

const ARGS: &str = "args";
const ALLOW: &str = "allow";
const EXCEPT: &str = "except";
const ALIAS: &str = "aka";
const GETTER: &str = "getter";
const SETTER: &str = "setter";
const SKIP: &str = "skip";
const INC_FOR_VEC: &str = "inc";
const SETTER_PREFIX: &str = "set_pre"; // TODO
const GETTER_PREFIX: &str = "get_pre"; // TODO
const SETTER_PREFIX_DEFAULT: &str = "with";
const GETTER_PREFIX_DEFAULT: &str = "nth";
const PRIMITIVE_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "bool",
    "char", "unit", "f32", "f64",
];

pub(crate) enum Fns {
    Setter(Tys),
    Getter(Tys),
}

pub(crate) enum Tys {
    Basic,
    Ref,
    String,
    Vec,
    VecInc,
    VecString,
    VecStringInc,
    Option,
    OptionAsRef,
    OptionVec,
    OptionString,
    OptionVecString,
}

#[proc_macro_derive(Builder, attributes(args))]
pub fn derive(x: TokenStream) -> TokenStream {
    let st = parse_macro_input!(x as DeriveInput);
    let expanded = build_expanded(st);
    TokenStream::from(expanded)
}

fn build_expanded(st: DeriveInput) -> proc_macro2::TokenStream {
    // generate
    let code = match &st.data {
        Data::Struct(data) => generate_from_struct(data),
        Data::Enum(_) | Data::Union(_) => panic!("`aksr` Builder can only be derived for struct"),
    };

    // attrs
    let (struct_name, (impl_generics, ty_generics, where_clause)) =
        (&st.ident, &st.generics.split_for_impl());

    // token stream
    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #code
        }
    }
}

fn generate_from_struct(data_struct: &DataStruct) -> proc_macro2::TokenStream {
    // code container
    let mut codes = quote! {};

    // traverse
    for (idx, field) in data_struct.fields.iter().enumerate() {
        // build rules from field
        let rules = Rules::from(field);

        // generate code based on field
        match &field.ty {
            Type::Path(type_path) => {
                if let Some(last_segment) = type_path.path.segments.last() {
                    match last_segment.ident.to_string().as_str() {
                        "String" => {
                            generate(
                                field,
                                &rules,
                                idx,
                                None,
                                &mut codes,
                                Fns::Setter(Tys::String),
                            );
                            generate(
                                field,
                                &rules,
                                idx,
                                None,
                                &mut codes,
                                Fns::Getter(Tys::String),
                            );
                        }

                        "Vec" => {
                            // Vec<T> -> &[T]
                            if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                                if let Some(arg) = args.args.first() {
                                    if let GenericArgument::Type(ty) = arg {
                                        if let Type::Path(type_path) = &ty {
                                            if let Some(last_segment) =
                                                type_path.path.segments.last()
                                            {
                                                let ident = &last_segment.ident;

                                                // Vec<String> -> &[&str]
                                                if ident == "String" {
                                                    generate(
                                                        field,
                                                        &rules,
                                                        idx,
                                                        None,
                                                        &mut codes,
                                                        Fns::Setter(Tys::VecString),
                                                    );

                                                    // increment ver
                                                    generate(
                                                        field,
                                                        &rules,
                                                        idx,
                                                        None,
                                                        &mut codes,
                                                        Fns::Setter(Tys::VecStringInc),
                                                    );
                                                } else {
                                                    // setters
                                                    generate(
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(Tys::Vec),
                                                    );

                                                    // setters inc
                                                    generate(
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(Tys::VecInc),
                                                    );
                                                }

                                                // getters: Vec<T> -> &[T]
                                                generate(
                                                    field,
                                                    &rules,
                                                    idx,
                                                    Some(arg),
                                                    &mut codes,
                                                    Fns::Getter(Tys::Vec),
                                                );
                                            }
                                        } else {
                                            // Vec<T> -> &[T]
                                            // setters
                                            generate(
                                                field,
                                                &rules,
                                                idx,
                                                Some(arg),
                                                &mut codes,
                                                Fns::Setter(Tys::Vec),
                                            );

                                            // setters inc
                                            generate(
                                                field,
                                                &rules,
                                                idx,
                                                Some(arg),
                                                &mut codes,
                                                Fns::Setter(Tys::VecInc),
                                            );
                                            // getters: Vec<T> -> &[T]
                                            generate(
                                                field,
                                                &rules,
                                                idx,
                                                Some(arg),
                                                &mut codes,
                                                Fns::Getter(Tys::Vec),
                                            );
                                        }
                                    }
                                }
                            }
                        }

                        "Option" => {
                            // Option<T>
                            // - T => String => &str
                            // - T => Vec<U> => &[U]
                            //   - U => String => &str
                            if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                                if let Some(arg) = &args.args.first() {
                                    if let GenericArgument::Type(ty) = arg {
                                        if let Type::Path(type_path) = &ty {
                                            if let Some(last_segment) =
                                                type_path.path.segments.last()
                                            {
                                                let ident = &last_segment.ident;
                                                // T => Vec<U> => &[U]
                                                if ident == "Vec" {
                                                    if let PathArguments::AngleBracketed(args) =
                                                        &last_segment.arguments
                                                    {
                                                        // U
                                                        if let Some(arg) = args.args.first() {
                                                            if let GenericArgument::Type(
                                                                Type::Path(type_path),
                                                            ) = arg
                                                            {
                                                                if let Some(last_segment) =
                                                                    type_path.path.segments.last()
                                                                {
                                                                    // U => String => &str
                                                                    // Option<Vec<String>> -> Option<&[&str]>
                                                                    if last_segment.ident
                                                                        == "String"
                                                                    {
                                                                        generate(
                                                                            field,
                                                                            &rules,
                                                                            idx,
                                                                            None,
                                                                            &mut codes,
                                                                            Fns::Setter(Tys::OptionVecString),
                                                                        );
                                                                    } else {
                                                                        generate(
                                                                            field,
                                                                            &rules,
                                                                            idx,
                                                                            Some(arg),
                                                                            &mut codes,
                                                                            Fns::Setter(
                                                                                Tys::OptionVec,
                                                                            ),
                                                                        );
                                                                    }
                                                                }
                                                            } else {
                                                                generate(
                                                                    field,
                                                                    &rules,
                                                                    idx,
                                                                    Some(arg),
                                                                    &mut codes,
                                                                    Fns::Setter(Tys::OptionVec),
                                                                );
                                                            }

                                                            // getters: Option<Vec<T>> -> Option<&[T]>
                                                            generate(
                                                                field,
                                                                &rules,
                                                                idx,
                                                                Some(arg),
                                                                &mut codes,
                                                                Fns::Getter(Tys::OptionVec),
                                                            );
                                                        }
                                                    }
                                                } else if ident == "String" {
                                                    // T => String => &str
                                                    generate(
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(Tys::OptionString),
                                                    );

                                                    // getters: Option<String> -> Option<&str>
                                                    generate(
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Getter(Tys::OptionString),
                                                    );
                                                } else {
                                                    // T => T
                                                    generate(
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(Tys::Option),
                                                    );

                                                    if PRIMITIVE_TYPES
                                                        .contains(&ident.to_string().as_str())
                                                    {
                                                        // getters: Option<T> -> Option<T>
                                                        generate(
                                                            field,
                                                            &rules,
                                                            idx,
                                                            Some(arg),
                                                            &mut codes,
                                                            Fns::Getter(Tys::Option),
                                                        );
                                                    } else {
                                                        // getters: Option<T> -> Option<&T>
                                                        // Option<Box<T>>, Option<Option<T>>
                                                        generate(
                                                            field,
                                                            &rules,
                                                            idx,
                                                            Some(arg),
                                                            &mut codes,
                                                            Fns::Getter(Tys::OptionAsRef),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            //  others: Option<(u8, i8)>, Option<&'a str>,
                                            if let PathArguments::AngleBracketed(args) =
                                                &last_segment.arguments
                                            {
                                                if let Some(arg) = args.args.first() {
                                                    // setters
                                                    generate(
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(Tys::Option),
                                                    );

                                                    // getters
                                                    if let GenericArgument::Type(ty) = arg {
                                                        match ty {
                                                            Type::Reference(_) => {
                                                                // getters: Option<T> -> Option<T>
                                                                // Option<&'a str>
                                                                generate(
                                                                    field,
                                                                    &rules,
                                                                    idx,
                                                                    Some(arg),
                                                                    &mut codes,
                                                                    Fns::Getter(Tys::Option),
                                                                );
                                                            }
                                                            _ => {
                                                                // getters: Option<T> -> Option<&T>
                                                                // Option<(u8, i8)>
                                                                generate(
                                                                    field,
                                                                    &rules,
                                                                    idx,
                                                                    Some(arg),
                                                                    &mut codes,
                                                                    Fns::Getter(Tys::OptionAsRef),
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        xxx => {
                            generate(
                                field,
                                &rules,
                                idx,
                                None,
                                &mut codes,
                                Fns::Setter(Tys::Basic),
                            );
                            if PRIMITIVE_TYPES.contains(&xxx) {
                                generate(
                                    field,
                                    &rules,
                                    idx,
                                    None,
                                    &mut codes,
                                    Fns::Getter(Tys::Basic),
                                );
                            } else {
                                generate(
                                    field,
                                    &rules,
                                    idx,
                                    None,
                                    &mut codes,
                                    Fns::Getter(Tys::Ref),
                                );
                            }
                        }
                    }
                }
            }
            ty => {
                // setter
                generate(
                    field,
                    &rules,
                    idx,
                    None,
                    &mut codes,
                    Fns::Setter(Tys::Basic),
                );

                // getter
                match ty {
                    Type::Reference(_) => {
                        // &'a T or &'a mut T
                        generate(
                            field,
                            &rules,
                            idx,
                            None,
                            &mut codes,
                            Fns::Getter(Tys::Basic),
                        );
                    }
                    Type::Array(_) | Type::Tuple(_) => {
                        // array [T; n] and tuple (A, B, C, String)
                        generate(field, &rules, idx, None, &mut codes, Fns::Getter(Tys::Ref));
                    }
                    _ => {
                        // TODO: others
                        generate(field, &rules, idx, None, &mut codes, Fns::Getter(Tys::Ref));
                    }
                }
            }
        }
    }

    // token stream
    quote! {
        #codes
    }
}

fn generate(
    field: &Field,
    rules: &Rules,
    idx: usize,
    arg: Option<&GenericArgument>,
    codes: &mut proc_macro2::TokenStream,
    fn_type: Fns,
) {
    // setter_name & getter_name
    let (setter_name, getter_name) = rules.generate_setter_getter_names(field, idx); // (move inside????)

    // attrs
    let field_type = &field.ty;
    let field_name = field.ident.as_ref();
    let field_index = Index::from(idx);
    let field_access = field_name.map_or_else(|| quote! { #field_index }, |name| quote! { #name });

    // token stream
    let code = match fn_type {
        Fns::Setter(ty) => {
            if !rules.gen_setter {
                return;
            }
            match ty {
                Tys::Basic => {
                    quote! {
                        #[doc = concat!(" Sets the value of the `", stringify!(#field_access), "` field.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - The new value to be assigned to the field."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: #field_type) -> Self {
                            self.#field_access = x;
                            self
                        }
                    }
                }
                Tys::String => {
                    quote! {
                        #[doc = concat!(" Sets the value of the `", stringify!(#field_access), "` field from a string slice.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - A string slice that will be converted to a `String`."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: &str) -> Self {
                            self.#field_access = x.to_string();
                            self
                        }
                    }
                }
                Tys::Vec => {
                    let arg = arg.expect("Vec setter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Sets the value of the `", stringify!(#field_access), "` field from a slice.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - A slice of elements to be converted into a vector."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                            self.#field_access = x.to_vec();
                            self
                        }
                    }
                }
                Tys::VecInc if rules.inc_for_vec => {
                    let arg = arg.expect("VecInc setter requires a generic argument");
                    let setter_name = Ident::new(
                        &format!("{}_{}", setter_name, INC_FOR_VEC),
                        Span::call_site(),
                    );
                    quote! {
                        #[doc = concat!(" Appends values to the `", stringify!(#field_access), "` field.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - A slice of elements to be appended to the vector."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                            if self.#field_access.is_empty() {
                                self.#field_access = Vec::from(x);
                            } else {
                                self.#field_access.extend_from_slice(x);
                            }
                            self
                        }
                    }
                }
                Tys::VecString => {
                    quote! {
                        #[doc = concat!(" Sets the value of the `", stringify!(#field_access), "` field from a slice of string slices.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - A slice of string slices to be converted into a vector of `String`."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: &[&str]) -> Self {
                            self.#field_access = x.iter().map(|s| s.to_string()).collect();
                            self
                        }
                    }
                }
                Tys::VecStringInc if rules.inc_for_vec => {
                    let setter_name = Ident::new(
                        &format!("{}_{}", setter_name, INC_FOR_VEC),
                        Span::call_site(),
                    );
                    quote! {
                        #[doc = concat!(" Appends string values to the `", stringify!(#field_access), "` field.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - A slice of string slices to be appended as `String` values."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: &[&str]) -> Self {
                            if self.#field_access.is_empty() {
                                self.#field_access = x.iter().map(|s| s.to_string()).collect();
                            } else {
                                let mut x = x.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                                self.#field_access.append(&mut x);
                            }
                            self
                        }
                    }
                }
                Tys::Option => {
                    quote! {
                        #[doc = concat!(" Sets the value of the optional `", stringify!(#field_access), "` field.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - The value to be wrapped in `Some` and assigned to the field."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: #arg) -> Self {
                            self.#field_access = Some(x);
                            self
                        }
                    }
                }
                Tys::OptionVec => {
                    let arg = arg.expect("OptionVec setter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Sets the value of the optional `", stringify!(#field_access), "` field from a slice.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - A slice of elements to be converted into a vector and wrapped in `Some`."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                            self.#field_access = Some(x.to_vec());
                            self
                        }
                    }
                }
                Tys::OptionVecString => {
                    quote! {
                        #[doc = concat!(" Sets the value of the optional `", stringify!(#field_access), "` field from a slice of string slices.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - A slice of string slices to be converted into a vector of `String` and wrapped in `Some`."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: &[&str]) -> Self {
                            self.#field_access = Some(x.iter().map(|s| s.to_string()).collect());
                            self
                        }
                    }
                }
                Tys::OptionString => {
                    quote! {
                        #[doc = concat!(" Sets the value of the optional `", stringify!(#field_access), "` field from a string slice.")]
                        #[doc = " "]
                        #[doc = " # Arguments"]
                        #[doc = " "]
                        #[doc = " * `x` - A string slice to be converted into a `String` and wrapped in `Some`."]
                        #[doc = " "]
                        #[doc = " # Returns"]
                        #[doc = " "]
                        #[doc = " Returns `Self` to allow method chaining."]
                        pub fn #setter_name(mut self, x: &str) -> Self {
                            self.#field_access = Some(x.to_string());
                            self
                        }
                    }
                }
                _ => quote! {},
            }
        }
        Fns::Getter(ty) => {
            if !rules.gen_getter {
                return;
            }
            match ty {
                Tys::Basic => {
                    quote! {
                        #[doc = concat!(" Returns the value of the `", stringify!(#field_access), "` field.")]
                        pub fn #getter_name(&self) -> #field_type {
                            self.#field_access
                        }
                    }
                }
                Tys::Ref => {
                    quote! {
                        #[doc = concat!(" Returns a reference to the `", stringify!(#field_access), "` field.")]
                        pub fn #getter_name(&self) -> &#field_type {
                            &self.#field_access
                        }
                    }
                }
                Tys::String => {
                    quote! {
                        #[doc = concat!(" Returns a string slice of the `", stringify!(#field_access), "` field.")]
                        pub fn #getter_name(&self) -> &str {
                            &self.#field_access
                        }
                    }
                }
                Tys::Vec => {
                    let arg = arg.expect("Vec getter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Returns a slice of the `", stringify!(#field_access), "` field.")]
                        pub fn #getter_name(&self) -> &[#arg] {
                            &self.#field_access
                        }
                    }
                }
                Tys::Option => {
                    let arg = arg.expect("Option getter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Returns the value of the optional `", stringify!(#field_access), "` field.")]
                        pub fn #getter_name(&self) -> Option<#arg> {
                            self.#field_access
                        }
                    }
                }
                Tys::OptionAsRef => {
                    let arg = arg.expect("OptionAsRef getter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Returns an optional reference to the `", stringify!(#field_access), "` field.")]
                        pub fn #getter_name(&self) -> Option<&#arg> {
                            self.#field_access.as_ref()
                        }
                    }
                }
                Tys::OptionString => {
                    quote! {
                        #[doc = concat!(" Returns an optional string slice of the `", stringify!(#field_access), "` field.")]
                        pub fn #getter_name(&self) -> Option<&str> {
                            self.#field_access.as_deref()
                        }
                    }
                }
                Tys::OptionVec => {
                    let arg = arg.expect("OptionVec getter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Returns an optional slice of the `", stringify!(#field_access), "` field.")]
                        pub fn #getter_name(&self) -> Option<&[#arg]> {
                            self.#field_access.as_deref()
                        }
                    }
                }
                _ => quote! {},
            }
        }
    };

    // append
    codes.extend(code);
}
