//! # aksr
//!
//! A Rust derive macro that automatically generates **[Builder Lite](https://matklad.github.io/2022/05/29/builder-lite.html)** pattern methods for structs.
//!
//! The struct itself acts as the builder, eliminating the need for a separate builder type. This is especially useful for rapidly evolving application code.
//!
//! **Requirements:** Struct must implement `Default` or have a `new()` method.
//!
//! ## Examples
//!
//! See the [`examples`](https://github.com/jamjamjon/aksr/tree/main/examples) directory:
//! - [`examples/rect.rs`](examples/rect.rs) - Named struct with all features
//! - [`examples/color.rs`](examples/color.rs) - Tuple struct with all features
//!
//! Run examples with `cargo run --example rect` or `cargo run --example color`.
//!
//! To see the generated code, use `cargo install cargo-expand` and run `cargo expand --example rect`.
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

// attributes
const ARGS: &str = "args";
const ALLOW: &str = "allow";
const EXCEPT: &str = "except";
const ALIAS: &str = "alias";
#[allow(dead_code)]
#[deprecated(since = "0.1.0", note = "use `alias` instead")]
const ALIAS_DEPRECATED: &str = "aka";
const GETTER: &str = "getter";
const SETTER: &str = "setter";
const SKIP: &str = "skip";
const EXTEND: &str = "extend";
#[allow(dead_code)]
#[deprecated(since = "0.1.0", note = "use `extend` instead")]
const EXTEND_DEPRECATED: &str = "inc";
const SETTER_PREFIX: &str = "setter_prefix";
const GETTER_PREFIX: &str = "getter_prefix";
const VISIBILITY: &str = "visibility";
const GETTER_VISIBILITY: &str = "getter_visibility";
const SETTER_VISIBILITY: &str = "setter_visibility";
const INLINE: &str = "inline";
const GETTER_INLINE: &str = "getter_inline";
const SETTER_INLINE: &str = "setter_inline";
const SETTER_PREFIX_DEFAULT: &str = "with";
const GETTER_PREFIX_DEFAULT: &str = "nth";
const PRIMITIVE_TYPES: &[&str] = &[
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "bool",
    "char",
    "unit",
    "f32",
    "f64",
    "f16",
    "bf16",
    "half::f16",
    "half::bf16",
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
    VecStringOwned,
    VecStringInc,
    VecStringIncOwned,
    Option,
    OptionOption,
    OptionAsRef,
    OptionVec,
    OptionString,
    OptionVecString,
    OptionVecStringOwned,
}

#[proc_macro_derive(Builder, attributes(args))]
pub fn derive(x: TokenStream) -> TokenStream {
    let st = parse_macro_input!(x as DeriveInput);
    let expanded = build_expanded(st);
    TokenStream::from(expanded)
}

fn build_expanded(st: DeriveInput) -> proc_macro2::TokenStream {
    // attrs
    let (struct_name, (impl_generics, ty_generics, where_clause)) =
        (&st.ident, &st.generics.split_for_impl());

    // generate
    let code = match &st.data {
        Data::Struct(data) => generate_from_struct(struct_name, data),
        Data::Enum(_) | Data::Union(_) => panic!("`aksr` Builder can only be derived for struct"),
    };

    // token stream
    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #code
        }
    }
}

fn generate_from_struct(struct_name: &Ident, data_struct: &DataStruct) -> proc_macro2::TokenStream {
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
                                struct_name,
                                field,
                                &rules,
                                idx,
                                None,
                                &mut codes,
                                Fns::Setter(Tys::String),
                            );
                            generate(
                                struct_name,
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
                                                    // setter: &[&str]
                                                    generate(
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        None,
                                                        &mut codes,
                                                        Fns::Setter(Tys::VecString),
                                                    );

                                                    // setter: &[String] (owned)
                                                    generate(
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        None,
                                                        &mut codes,
                                                        Fns::Setter(Tys::VecStringOwned),
                                                    );

                                                    // increment ver: &[&str]
                                                    generate(
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        None,
                                                        &mut codes,
                                                        Fns::Setter(Tys::VecStringInc),
                                                    );

                                                    // increment ver: &[String] (owned)
                                                    generate(
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        None,
                                                        &mut codes,
                                                        Fns::Setter(Tys::VecStringIncOwned),
                                                    );
                                                } else {
                                                    // setters
                                                    generate(
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(Tys::Vec),
                                                    );

                                                    // setters inc
                                                    generate(
                                                        struct_name,
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
                                                    struct_name,
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
                                                struct_name,
                                                field,
                                                &rules,
                                                idx,
                                                Some(arg),
                                                &mut codes,
                                                Fns::Setter(Tys::Vec),
                                            );

                                            // setters inc
                                            generate(
                                                struct_name,
                                                field,
                                                &rules,
                                                idx,
                                                Some(arg),
                                                &mut codes,
                                                Fns::Setter(Tys::VecInc),
                                            );
                                            // getters: Vec<T> -> &[T]
                                            generate(
                                                struct_name,
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
                                                                        // setter: &[&str]
                                                                        generate(
                                                                            struct_name,
                                                                            field,
                                                                            &rules,
                                                                            idx,
                                                                            None,
                                                                            &mut codes,
                                                                            Fns::Setter(Tys::OptionVecString),
                                                                        );

                                                                        // setter: &[String] (owned)
                                                                        generate(
                                                                            struct_name,
                                                                            field,
                                                                            &rules,
                                                                            idx,
                                                                            None,
                                                                            &mut codes,
                                                                            Fns::Setter(Tys::OptionVecStringOwned),
                                                                        );
                                                                    } else {
                                                                        generate(
                                                                            struct_name,
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
                                                                    struct_name,
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
                                                                struct_name,
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
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(Tys::OptionString),
                                                    );

                                                    // getters: Option<String> -> Option<&str>
                                                    generate(
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Getter(Tys::OptionString),
                                                    );
                                                } else {
                                                    // T => T
                                                    // Check if arg is itself an Option type (for nested Option<Option<T>>)
                                                    let is_nested_option =
                                                        if let GenericArgument::Type(Type::Path(
                                                            inner_type_path,
                                                        )) = arg
                                                        {
                                                            if let Some(inner_segment) =
                                                                inner_type_path.path.segments.last()
                                                            {
                                                                inner_segment.ident == "Option"
                                                            } else {
                                                                false
                                                            }
                                                        } else {
                                                            false
                                                        };

                                                    generate(
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(if is_nested_option {
                                                            Tys::OptionOption
                                                        } else {
                                                            Tys::Option
                                                        }),
                                                    );

                                                    if PRIMITIVE_TYPES
                                                        .contains(&ident.to_string().as_str())
                                                    {
                                                        // getters: Option<T> -> Option<T>
                                                        generate(
                                                            struct_name,
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
                                                            struct_name,
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
                                                    // Check if arg is itself an Option type (for nested Option<Option<T>>)
                                                    let is_nested_option =
                                                        if let GenericArgument::Type(Type::Path(
                                                            inner_type_path,
                                                        )) = arg
                                                        {
                                                            if let Some(inner_segment) =
                                                                inner_type_path.path.segments.last()
                                                            {
                                                                inner_segment.ident == "Option"
                                                            } else {
                                                                false
                                                            }
                                                        } else {
                                                            false
                                                        };

                                                    generate(
                                                        struct_name,
                                                        field,
                                                        &rules,
                                                        idx,
                                                        Some(arg),
                                                        &mut codes,
                                                        Fns::Setter(if is_nested_option {
                                                            Tys::OptionOption
                                                        } else {
                                                            Tys::Option
                                                        }),
                                                    );

                                                    // getters
                                                    if let GenericArgument::Type(ty) = arg {
                                                        match ty {
                                                            Type::Reference(_) => {
                                                                // getters: Option<T> -> Option<T>
                                                                // Option<&'a str>
                                                                generate(
                                                                    struct_name,
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
                                                                    struct_name,
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
                                struct_name,
                                field,
                                &rules,
                                idx,
                                None,
                                &mut codes,
                                Fns::Setter(Tys::Basic),
                            );
                            if PRIMITIVE_TYPES.contains(&xxx) {
                                generate(
                                    struct_name,
                                    field,
                                    &rules,
                                    idx,
                                    None,
                                    &mut codes,
                                    Fns::Getter(Tys::Basic),
                                );
                            } else {
                                generate(
                                    struct_name,
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
                    struct_name,
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
                            struct_name,
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
                        generate(
                            struct_name,
                            field,
                            &rules,
                            idx,
                            None,
                            &mut codes,
                            Fns::Getter(Tys::Ref),
                        );
                    }
                    _ => {
                        // TODO: others
                        generate(
                            struct_name,
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

    // token stream
    quote! {
        #codes
    }
}

fn generate(
    struct_name: &Ident,
    field: &Field,
    rules: &Rules,
    idx: usize,
    arg: Option<&GenericArgument>,
    codes: &mut proc_macro2::TokenStream,
    fn_type: Fns,
) {
    // setter_name & getter_name
    let (setter_name, getter_name) = rules.generate_setter_getter_names(field, idx); // (move inside????)

    // visibility tokens
    let setter_visibility = rules.setter_visibility_token();
    let getter_visibility = rules.getter_visibility_token();

    // inline tokens
    let setter_inline = rules.setter_inline_token();
    let getter_inline = rules.getter_inline_token();

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
                        #[doc = concat!(" Sets the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - The new value to be assigned"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(value);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: #field_type) -> Self {
                            self.#field_access = x;
                            self
                        }
                    }
                }
                Tys::String => {
                    quote! {
                        #[doc = concat!(" Sets the `", stringify!(#field_access), "` field from a string slice.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A string slice that will be converted to `String`"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(\"value\");")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: &str) -> Self {
                            self.#field_access = x.to_string();
                            self
                        }
                    }
                }
                Tys::Vec => {
                    let arg = arg.expect("Vec setter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Sets the `", stringify!(#field_access), "` field from a slice.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of elements to be converted into a vector"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " If the slice is empty, the field remains unchanged."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(&[item1, item2]);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: &[#arg]) -> Self {
                            if !x.is_empty() {
                                self.#field_access = x.to_vec();
                            }
                            self
                        }
                    }
                }
                Tys::VecInc if rules.inc_for_vec => {
                    let arg = arg.expect("VecInc setter requires a generic argument");
                    let setter_name =
                        Ident::new(&format!("{setter_name}_{EXTEND}"), Span::call_site());
                    quote! {
                        #[doc = concat!(" Appends elements to the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of elements to append to the vector"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(&[item1, item2]);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: &[#arg]) -> Self {
                            if !x.is_empty() {
                                if self.#field_access.is_empty() {
                                    self.#field_access = Vec::from(x);
                                } else {
                                    self.#field_access.extend_from_slice(x);
                                }
                            }
                            self
                        }
                    }
                }
                Tys::VecString => {
                    quote! {
                        #[doc = concat!(" Sets the `", stringify!(#field_access), "` field from a slice of string slices.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of string slices that will be automatically converted to `Vec<String>`"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " If the slice is empty, the field remains unchanged."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(&[\"str1\", \"str2\"]);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: &[&str]) -> Self {
                            if !x.is_empty() {
                                self.#field_access = x.iter().map(|s| s.to_string()).collect();
                            }
                            self
                        }
                    }
                }
                Tys::VecStringOwned => {
                    let setter_name_owned =
                        Ident::new(&format!("{setter_name}_owned"), Span::call_site());
                    quote! {
                        #[doc = concat!(" Sets the `", stringify!(#field_access), "` field from a slice of owned strings.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of `String` to be cloned into the vector"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " This method is useful when you already have a `Vec<String>` and want to avoid converting to `&[&str]`. "]
                        #[doc = " If the slice is empty, the field remains unchanged."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let strings = vec![String::from(\"a\"), String::from(\"b\")];")]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name_owned), "(&strings);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name_owned(mut self, x: &[String]) -> Self {
                            if !x.is_empty() {
                                self.#field_access = x.to_vec();
                            }
                            self
                        }
                    }
                }
                Tys::VecStringInc if rules.inc_for_vec => {
                    let setter_name =
                        Ident::new(&format!("{setter_name}_{EXTEND}"), Span::call_site());
                    quote! {
                        #[doc = concat!(" Appends string values to the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of string slices to append to the vector"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(&[\"str1\", \"str2\"]);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: &[&str]) -> Self {
                            if !x.is_empty() {
                                if self.#field_access.is_empty() {
                                    self.#field_access = x.iter().map(|s| s.to_string()).collect();
                                } else {
                                    let mut x = x.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                                    self.#field_access.append(&mut x);
                                }
                            }
                            self
                        }
                    }
                }
                Tys::VecStringIncOwned if rules.inc_for_vec => {
                    let setter_name_owned =
                        Ident::new(&format!("{setter_name}_{EXTEND}_owned"), Span::call_site());
                    quote! {
                        #[doc = concat!(" Appends owned string values to the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of `String` to append to the vector"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " This method is useful when you already have a `Vec<String>` and want to avoid converting to `&[&str]`."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let more = vec![String::from(\"c\")];")]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name_owned), "(&more);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name_owned(mut self, x: &[String]) -> Self {
                            if !x.is_empty() {
                                if self.#field_access.is_empty() {
                                    self.#field_access = x.to_vec();
                                } else {
                                    self.#field_access.extend_from_slice(x);
                                }
                            }
                            self
                        }
                    }
                }
                Tys::Option => {
                    quote! {
                        #[doc = concat!(" Sets the optional `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - The value that will be automatically wrapped in `Some`"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " The value is automatically wrapped in `Some`, so you don't need to pass `Some(value)`."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(value);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: #arg) -> Self {
                            self.#field_access = Some(x);
                            self
                        }
                    }
                }
                Tys::OptionOption => {
                    quote! {
                        #[doc = concat!(" Sets the optional `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - An `Option` value to be assigned. If `None`, the field remains unchanged."]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(Some(value));")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: #arg) -> Self {
                            if x.is_some() {
                                self.#field_access = Some(x);
                            }
                            self
                        }
                    }
                }
                Tys::OptionVec => {
                    let arg = arg.expect("OptionVec setter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Sets the optional `", stringify!(#field_access), "` field from a slice.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of elements that will be automatically converted to a vector and wrapped in `Some`"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " If the slice is empty, the field remains unchanged. Otherwise, it's automatically converted to `Vec` and wrapped in `Some`."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(&[item1, item2]);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: &[#arg]) -> Self {
                            if !x.is_empty() {
                                self.#field_access = Some(x.to_vec());
                            }
                            self
                        }
                    }
                }
                Tys::OptionVecString => {
                    quote! {
                        #[doc = concat!(" Sets the optional `", stringify!(#field_access), "` field from a slice of string slices.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of string slices that will be automatically converted to `Vec<String>` and wrapped in `Some`"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " If the slice is empty, the field remains unchanged. Otherwise, it's automatically converted to `Vec<String>` and wrapped in `Some`."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(&[\"str1\", \"str2\"]);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: &[&str]) -> Self {
                            if !x.is_empty() {
                                self.#field_access = Some(x.iter().map(|s| s.to_string()).collect());
                            }
                            self
                        }
                    }
                }
                Tys::OptionVecStringOwned => {
                    let setter_name_owned =
                        Ident::new(&format!("{setter_name}_owned"), Span::call_site());
                    quote! {
                        #[doc = concat!(" Sets the optional `", stringify!(#field_access), "` field from a slice of owned strings.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A slice of `String` that will be automatically cloned into a vector and wrapped in `Some`"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " This method is useful when you already have a `Vec<String>` and want to avoid converting to `&[&str]`. "]
                        #[doc = " If the slice is empty, the field remains unchanged. Otherwise, it's automatically wrapped in `Some`."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let strings = vec![String::from(\"a\")];")]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name_owned), "(&strings);")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name_owned(mut self, x: &[String]) -> Self {
                            if !x.is_empty() {
                                self.#field_access = Some(x.to_vec());
                            }
                            self
                        }
                    }
                }
                Tys::OptionString => {
                    quote! {
                        #[doc = concat!(" Sets the optional `", stringify!(#field_access), "` field from a string slice.")]
                        #[doc = ""]
                        #[doc = " # Arguments"]
                        #[doc = ""]
                        #[doc = " * `x` - A string slice that will be automatically converted to `String` and wrapped in `Some`"]
                        #[doc = ""]
                        #[doc = " # Returns"]
                        #[doc = ""]
                        #[doc = " Returns `Self` for method chaining."]
                        #[doc = ""]
                        #[doc = " # Note"]
                        #[doc = ""]
                        #[doc = " The string slice is automatically converted to `String` and wrapped in `Some`."]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!("let obj = ", stringify!(#struct_name), "::default().", stringify!(#setter_name), "(\"value\");")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #setter_inline
                        #setter_visibility fn #setter_name(mut self, x: &str) -> Self {
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
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default();")]
                        #[doc = concat!(" let value = obj.", stringify!(#getter_name), "();")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #getter_inline
                        #getter_visibility fn #getter_name(&self) -> #field_type {
                            self.#field_access
                        }
                    }
                }
                Tys::Ref => {
                    quote! {
                        #[doc = concat!(" Returns a reference to the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default();")]
                        #[doc = concat!(" let value = obj.", stringify!(#getter_name), "();")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #getter_inline
                        #getter_visibility fn #getter_name(&self) -> &#field_type {
                            &self.#field_access
                        }
                    }
                }
                Tys::String => {
                    quote! {
                        #[doc = concat!(" Returns a string slice of the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default();")]
                        #[doc = concat!(" let value = obj.", stringify!(#getter_name), "();")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #getter_inline
                        #getter_visibility fn #getter_name(&self) -> &str {
                            &self.#field_access
                        }
                    }
                }
                Tys::Vec => {
                    let arg = arg.expect("Vec getter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Returns a slice view of the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default();")]
                        #[doc = concat!(" let items = obj.", stringify!(#getter_name), "();")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #getter_inline
                        #getter_visibility fn #getter_name(&self) -> &[#arg] {
                            &self.#field_access
                        }
                    }
                }
                Tys::Option => {
                    let arg = arg.expect("Option getter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Returns the value of the optional `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default();")]
                        #[doc = concat!(" let value = obj.", stringify!(#getter_name), "();")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #getter_inline
                        #getter_visibility fn #getter_name(&self) -> Option<#arg> {
                            self.#field_access
                        }
                    }
                }
                Tys::OptionAsRef => {
                    let arg = arg.expect("OptionAsRef getter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Returns an optional reference to the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default();")]
                        #[doc = concat!(" let value = obj.", stringify!(#getter_name), "();")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #getter_inline
                        #getter_visibility fn #getter_name(&self) -> Option<&#arg> {
                            self.#field_access.as_ref()
                        }
                    }
                }
                Tys::OptionString => {
                    quote! {
                        #[doc = concat!(" Returns an optional string slice of the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default();")]
                        #[doc = concat!(" let value = obj.", stringify!(#getter_name), "();")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #getter_inline
                        #getter_visibility fn #getter_name(&self) -> Option<&str> {
                            self.#field_access.as_deref()
                        }
                    }
                }
                Tys::OptionVec => {
                    let arg = arg.expect("OptionVec getter requires a generic argument");
                    quote! {
                        #[doc = concat!(" Returns an optional slice view of the `", stringify!(#field_access), "` field.")]
                        #[doc = ""]
                        #[doc = " # Example"]
                        #[doc = ""]
                        #[doc = " ```"]
                        #[doc = concat!(" let obj = ", stringify!(#struct_name), "::default();")]
                        #[doc = concat!(" let items = obj.", stringify!(#getter_name), "();")]
                        #[doc = " ```"]
                        #[doc = ""]
                        #[doc = " ---"]
                        #[doc = " *Generated by `aksr` - Builder pattern macro*"]
                        #getter_inline
                        #getter_visibility fn #getter_name(&self) -> Option<&[#arg]> {
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
