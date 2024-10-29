use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, GenericArgument, Index, PathArguments,
    Type,
};

mod misc;
use misc::{Fns, Rules, Tys};

const ARGS: &str = "args";
const ALIAS: &str = "alias";
const GETTER: &str = "getter";
const SETTER: &str = "setter";
const SETTER_PREFIX: &str = "setter_prefix";
const GETTER_PREFIX: &str = "getter_prefix";
const INC_FOR_VEC: &str = "inc";
const SETTER_PREFIX_DEFAULT: &str = "with";
const GETTER_PREFIX_DEFAULT: &str = "nth";
const PRIMITIVE_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "bool",
    "char", "unit", "f32", "f64",
];

#[proc_macro_derive(Builder, attributes(args))]
pub fn derive(x: TokenStream) -> TokenStream {
    let st = parse_macro_input!(x as DeriveInput);
    let expanded = build_expanded(st);
    TokenStream::from(expanded)
}

fn build_expanded(st: DeriveInput) -> proc_macro2::TokenStream {
    // generate code
    let code = match &st.data {
        Data::Struct(data) => generate_from_struct(data),
        Data::Enum(_) | Data::Union(_) => panic!("Builder(aksr) can only be derived for struct"),
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
                        pub fn #setter_name(mut self, x: #field_type) -> Self {
                            self.#field_access = x;
                            self
                        }
                    }
                }
                Tys::String => {
                    quote! {
                        pub fn #setter_name(mut self, x: &str) -> Self {
                            self.#field_access = x.to_string();
                            self
                        }
                    }
                }
                Tys::Vec => {
                    let arg = arg.expect("Vec setter requires a generic argument");
                    quote! {
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
                        pub fn #setter_name(mut self, x: #arg) -> Self {
                            self.#field_access = Some(x);
                            self
                        }
                    }
                }
                Tys::OptionVec => {
                    let arg = arg.expect("OptionVec setter requires a generic argument");
                    quote! {
                        pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                            self.#field_access = Some(x.to_vec());
                            self
                        }
                    }
                }
                Tys::OptionVecString => {
                    quote! {
                        pub fn #setter_name(mut self, x: &[&str]) -> Self {
                            self.#field_access = Some(x.iter().map(|s| s.to_string()).collect());
                            self
                        }
                    }
                }
                Tys::OptionString => {
                    quote! {
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
                        pub fn #getter_name(&self) -> #field_type {
                            self.#field_access
                        }
                    }
                }
                Tys::Ref => {
                    quote! {
                        pub fn #getter_name(&self) -> &#field_type {
                            &self.#field_access
                        }
                    }
                }
                Tys::String => {
                    quote! {
                        pub fn #getter_name(&self) -> &str {
                            &self.#field_access
                        }
                    }
                }
                Tys::Vec => {
                    let arg = arg.expect("Vec getter requires a generic argument");
                    quote! {
                        pub fn #getter_name(&self) -> &[#arg] {
                            &self.#field_access
                        }
                    }
                }
                Tys::Option => {
                    let arg = arg.expect("Option getter requires a generic argument");
                    quote! {
                        pub fn #getter_name(&self) -> Option<#arg> {
                            self.#field_access
                        }
                    }
                }
                Tys::OptionAsRef => {
                    let arg = arg.expect("OptionAsRef getter requires a generic argument");
                    quote! {
                        pub fn #getter_name(&self) -> Option<&#arg> {
                            self.#field_access.as_ref()
                        }
                    }
                }
                Tys::OptionString => {
                    quote! {
                        pub fn #getter_name(&self) -> Option<&str> {
                            self.#field_access.as_deref()
                        }
                    }
                }
                Tys::OptionVec => {
                    let arg = arg.expect("OptionVec getter requires a generic argument");
                    quote! {
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
