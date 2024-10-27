use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Expr, GenericArgument, Lit, Meta,
    Token, Type,
};

const ARGS: &str = "args";
const ALIAS: &str = "alias";
const GETTER: &str = "getter";
const SETTER: &str = "setter";
const PREFIX: &str = "prefix";
const INC_FOR_VEC: &str = "inc";

#[proc_macro_derive(Builder, attributes(args))]
pub fn derive(x: TokenStream) -> TokenStream {
    let st = parse_macro_input!(x as DeriveInput);
    let expanded = build_expanded(st);
    TokenStream::from(expanded)
}

struct Rules {
    pub alias: Option<Ident>,
    pub inc_for_vec: bool,
    pub prefix_setter: String,
    pub gen_getter: bool,
    pub gen_setter: bool,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            alias: None,
            inc_for_vec: false,
            prefix_setter: "with".into(),
            gen_getter: true,
            gen_setter: true,
        }
    }
}

impl Rules {
    fn gen_setter_name(&self, field_name: &Ident) -> Ident {
        let setter_name = match &self.alias {
            None => format!("{}_{}", self.prefix_setter, field_name),
            Some(alias) => format!("{}_{}", self.prefix_setter, alias),
        };
        Ident::new(&setter_name, Span::call_site())
    }
}

fn build_expanded(st: DeriveInput) -> proc_macro2::TokenStream {
    // basics
    let fields = match &st.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("Builder can only be derived for struct"),
    };
    let struct_name = &st.ident;
    let (impl_generics, ty_generics, where_clause) = &st.generics.split_for_impl();

    // code container
    let mut fn_setters = quote! {};
    let mut fn_getters = quote! {};

    // traverse
    for field in fields {
        let field_type = &field.ty;
        let field_name = field
            .ident
            .as_ref()
            .expect("This filed has no ident. Tuple struct is unsupported for now.");

        // Parse field attrs
        let rules = parse_field_attributes(field);
        let setter_name = rules.gen_setter_name(field_name);

        // Generate setters and getters based on field type
        match &field_type {
            Type::Path(type_path) => {
                if let Some(last_segment) = type_path.path.segments.last() {
                    match last_segment.ident.to_string().as_str() {
                        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                        | "u64" | "u128" | "usize" | "bool" | "char" | "unit" | "f32" | "f64" => {
                            // setters
                            if rules.gen_setter {
                                fn_setters.extend(quote! {
                                    pub fn #setter_name(mut self, x:#field_type) -> Self {
                                        self.#field_name = x;
                                        self
                                    }
                                });
                            }
                            // getters
                            if rules.gen_getter {
                                fn_getters.extend(quote! {
                                    pub fn #field_name(&self) -> #field_type {
                                        self.#field_name
                                    }
                                });
                            }
                        }
                        "String" => {
                            // String -> &str

                            // setters
                            if rules.gen_setter {
                                fn_setters.extend(quote! {
                                    pub fn #setter_name(mut self, x: &str) -> Self {
                                        self.#field_name = x.to_string();
                                        self
                                    }
                                });
                            }

                            // getters
                            if rules.gen_getter {
                                fn_getters.extend(quote! {
                                    pub fn #field_name(&self) -> &str {
                                        &self.#field_name
                                    }
                                });
                            }
                        }
                        "Vec" => {
                            // Vec<T> -> &[T]
                            if let syn::PathArguments::AngleBracketed(args) =
                                &last_segment.arguments
                            {
                                if let Some(arg) = args.args.first() {
                                    if let syn::GenericArgument::Type(ty) = arg {
                                        if let Type::Path(type_path) = &ty {
                                            if let Some(last_segment) =
                                                type_path.path.segments.last()
                                            {
                                                let ident = &last_segment.ident;

                                                // Vec<String> -> &[&str]
                                                if ident == "String" {
                                                    // T => String => &str
                                                    if rules.gen_setter {
                                                        fn_setters.extend(quote! {
                                                        pub fn #setter_name(mut self, x: &[&str]) -> Self {
                                                            self.#field_name = x.iter().map(|s| s.to_string()).collect();
                                                            self
                                                        }
                                                    });
                                                    }

                                                    // increment ver
                                                    if rules.inc_for_vec {
                                                        let setter_name = Ident::new(
                                                            &format!("{}_inc", setter_name),
                                                            Span::call_site(),
                                                        );
                                                        // setters
                                                        if rules.gen_setter {
                                                            fn_setters.extend(quote! {
                                                            pub fn #setter_name(mut self, x: &[&str]) -> Self {
                                                                if self.#field_name.is_empty() {
                                                                    self.#field_name = x.iter().map(|s| s.to_string()).collect();
                                                                } else {
                                                                    let mut x = x.iter().map(|s| s.to_string()).collect();;
                                                                    self.#field_name.append(&mut x);
                                                                }
                                                                self
                                                            }
                                                        });
                                                        }
                                                    }
                                                } else {
                                                    // setters: override ver
                                                    if rules.gen_setter {
                                                        fn_setters.extend(quote! {
                                                        pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                                                            self.#field_name = x.to_vec();
                                                            self
                                                        }
                                                    });
                                                    }

                                                    // increment ver
                                                    if rules.inc_for_vec {
                                                        let setter_name = Ident::new(
                                                            &format!("{}_inc", setter_name),
                                                            Span::call_site(),
                                                        );
                                                        // setters
                                                        if rules.gen_setter {
                                                            fn_setters.extend(quote! {
                                                            pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                                                                if self.#field_name.is_empty() {
                                                                    self.#field_name = Vec::from(x);
                                                                } else {
                                                                    self.#field_name.extend_from_slice(x);
                                                                }
                                                                self
                                                            }
                                                        });
                                                        }
                                                    }
                                                }

                                                // getters: Vec<T> -> &[T]
                                                if rules.gen_getter {
                                                    fn_getters.extend(quote! {
                                                        pub fn #field_name(&self) -> &[#arg] {
                                                            &self.#field_name
                                                        }
                                                    });
                                                }
                                            }
                                        } else {
                                            // Vec<T> -> &[T]
                                            // setters: override ver
                                            if rules.gen_setter {
                                                fn_setters.extend(quote! {
                                                pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                                                    self.#field_name = x.to_vec();
                                                    self
                                                }
                                            });
                                            }

                                            // increment ver
                                            if rules.inc_for_vec {
                                                let setter_name = Ident::new(
                                                    &format!("{}_inc", setter_name),
                                                    Span::call_site(),
                                                );
                                                if rules.gen_setter {
                                                    fn_setters.extend(quote! {
                                                    pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                                                        if self.#field_name.is_empty() {
                                                            self.#field_name = Vec::from(x);
                                                        } else {
                                                            self.#field_name.extend_from_slice(x);
                                                        }
                                                        self
                                                    }
                                                });
                                                }
                                            }

                                            // getters: Vec<T> -> &[T]
                                            if rules.gen_getter {
                                                fn_getters.extend(quote! {
                                                    pub fn #field_name(&self) -> &[#arg] {
                                                        &self.#field_name
                                                    }
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        "Option" => {
                            // Option<T>
                            // - T => String => &str
                            // - T => Vec<U> => &[U]
                            //  - U => String => &str
                            if let syn::PathArguments::AngleBracketed(args) =
                                &last_segment.arguments
                            {
                                if let Some(arg) = &args.args.first() {
                                    if let syn::GenericArgument::Type(ty) = arg {
                                        if let Type::Path(type_path) = &ty {
                                            if let Some(last_segment) =
                                                type_path.path.segments.last()
                                            {
                                                let ident = &last_segment.ident;
                                                // T => Vec<U> => &[U]
                                                if ident == "Vec" {
                                                    if let syn::PathArguments::AngleBracketed(
                                                        args,
                                                    ) = &last_segment.arguments
                                                    {
                                                        // U
                                                        if let Some(arg) = args.args.first() {
                                                            //  setters
                                                            if rules.gen_setter {
                                                                if let GenericArgument::Type(
                                                                    Type::Path(type_path),
                                                                ) = arg
                                                                {
                                                                    if let Some(last_segment) =
                                                                        type_path
                                                                            .path
                                                                            .segments
                                                                            .last()
                                                                    {
                                                                        // U => String => &str
                                                                        // Option<Vec<String>> -> Option<&[&str]>
                                                                        if last_segment.ident
                                                                            == "String"
                                                                        {
                                                                            fn_setters.extend(quote! {
                                                                                pub fn #setter_name(mut self, x: &[&str]) -> Self {
                                                                                    self.#field_name = Some(x.iter().map(|s| s.to_string()).collect());
                                                                                    self
                                                                                }
                                                                            });
                                                                        } else {
                                                                            fn_setters.extend(quote! {
                                                                                pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                                                                                    self.#field_name = Some(x.to_vec());
                                                                                    self
                                                                                }
                                                                            });
                                                                        }
                                                                    }
                                                                } else {
                                                                    fn_setters.extend(quote! {
                                                                        pub fn #setter_name(mut self, x: &[#arg]) -> Self {
                                                                            self.#field_name = Some(x.to_vec());
                                                                            self
                                                                        }
                                                                    });
                                                                }
                                                            }

                                                            // getters: Option<Vec<T>> -> Option<&[T]>
                                                            if rules.gen_getter {
                                                                fn_getters.extend(quote! {
                                                                    pub fn #field_name(&self) -> Option<&[#arg]> {
                                                                        self.#field_name.as_deref()
                                                                    }
                                                                });
                                                            }
                                                        }
                                                    }
                                                } else if ident == "String" {
                                                    // T => String => &str
                                                    if rules.gen_setter {
                                                        fn_setters.extend(quote! {
                                                        pub fn #setter_name(mut self, x: &str) -> Self {
                                                            self.#field_name = Some(x.to_string());
                                                            self
                                                        }
                                                    });
                                                    }
                                                    // getters: Option<String> -> Option<&str>
                                                    if rules.gen_getter {
                                                        fn_getters.extend(quote! {
                                                            pub fn #field_name(&self) -> Option<&str> {
                                                                self.#field_name.as_deref()
                                                            }
                                                        });
                                                    }
                                                } else {
                                                    // T => T
                                                    // setters
                                                    if rules.gen_setter {
                                                        fn_setters.extend(quote! {
                                                        pub fn #setter_name(mut self, x: #arg) -> Self {
                                                            self.#field_name = Some(x);
                                                            self
                                                        }
                                                    });
                                                    }
                                                    match ident.to_string().as_str() {
                                                        "i8" | "i16" | "i32" | "i64" | "i128"
                                                        | "isize" | "u8" | "u16" | "u32"
                                                        | "u64" | "u128" | "usize" | "bool"
                                                        | "char" | "unit" | "f32" | "f64" => {
                                                            // getters: Option<T> -> Option<T>
                                                            if rules.gen_getter {
                                                                fn_getters.extend(quote! {
                                                                    pub fn #field_name(&self) -> Option<#arg> {
                                                                        self.#field_name
                                                                    }
                                                                });
                                                            }
                                                        }
                                                        _ => {
                                                            // getters: Option<T> -> Option<&T>
                                                            // Option<Box<T>>, Option<Option<T>>
                                                            if rules.gen_getter {
                                                                fn_getters.extend(quote! {
                                                                    pub fn #field_name(&self) -> Option<&#arg> {
                                                                        self.#field_name.as_ref()
                                                                    }
                                                                });
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            //  others: Option<(u8, i8)>, Option<&'a str>,
                                            if let syn::PathArguments::AngleBracketed(args) =
                                                &last_segment.arguments
                                            {
                                                if let Some(arg) = args.args.first() {
                                                    // setters
                                                    if rules.gen_setter {
                                                        fn_setters.extend(quote! {
                                                        pub fn #setter_name(mut self, x: #arg) -> Self {
                                                            self.#field_name = Some(x);
                                                            self
                                                        }
                                                    });
                                                    }

                                                    // getters
                                                    if let syn::GenericArgument::Type(ty) = arg {
                                                        match ty {
                                                            Type::Reference(_) => {
                                                                // getters: Option<T> -> Option<T>
                                                                // Option<&'a str>
                                                                if rules.gen_getter {
                                                                    fn_getters.extend(quote! {
                                                                        pub fn #field_name(&self) -> Option<#arg> {
                                                                            self.#field_name
                                                                        }
                                                                    });
                                                                }
                                                            }
                                                            _ => {
                                                                // getters: Option<T> -> Option<&T>
                                                                // Option<(u8, i8)>
                                                                if rules.gen_getter {
                                                                    fn_getters.extend(quote! {
                                                                        pub fn #field_name(&self) -> Option<&#arg> {
                                                                            self.#field_name.as_ref()
                                                                        }
                                                                    });
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
                        }
                        _ => {
                            // setters
                            if rules.gen_setter {
                                fn_setters.extend(quote! {
                                    pub fn #setter_name(mut self, x:#field_type) -> Self {
                                        self.#field_name = x;
                                        self
                                    }
                                });
                            }

                            // getters: HashMap -> &HashMap
                            if rules.gen_getter {
                                fn_getters.extend(quote! {
                                    pub fn #field_name(&self) -> &#field_type {
                                        &self.#field_name
                                    }
                                });
                            }
                        }
                    }
                }
            }
            Type::Reference(_) => {
                // A reference type: &'a T or &'a mut T.
                // setters
                if rules.gen_setter {
                    fn_setters.extend(quote! {
                        pub fn #setter_name(mut self, x: #field_type) -> Self {
                            self.#field_name = x;
                            self
                        }
                    });
                }
                // getters
                if rules.gen_getter {
                    fn_getters.extend(quote! {
                        pub fn #field_name(&self) -> #field_type {
                            self.#field_name
                        }
                    });
                }
            }
            Type::Array(_) | Type::Tuple(_) => {
                // A fixed size array type: [T; n].
                // A tuple type: (A, B, C, String).
                // setters
                if rules.gen_setter {
                    fn_setters.extend(quote! {
                        pub fn #setter_name(mut self, x: #field_type) -> Self {
                            self.#field_name = x;
                            self
                        }
                    });
                }
                // getters
                if rules.gen_getter {
                    fn_getters.extend(quote! {
                        pub fn #field_name(&self) -> &#field_type {
                            &self.#field_name
                        }
                    });
                }
            }
            _ => eprintln!("Unsupported field type: {:?}", field_type),
        }
    }

    // code
    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #fn_setters
            #fn_getters
        }
    }
}

// Function to parse boolean or string from attributes.
fn parse_bool_or_str(value: &Expr) -> bool {
    match value {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Bool(x) => x.value,
            Lit::Str(x) => matches!(
                x.value().to_lowercase().as_str(),
                "yes" | "true" | "t" | "y"
            ),
            _ => false,
        },
        _ => false,
    }
}

fn parse_field_attributes(field: &syn::Field) -> Rules {
    let mut rules = Rules::default();
    if let Some(attr) = &field.attrs.first() {
        if attr.path().is_ident(ARGS) {
            let nested = match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            {
                Ok(x) => x,
                Err(err) => panic!("{}", err),
            };
            for meta in &nested {
                match meta {
                    Meta::NameValue(name_value) => {
                        match name_value
                            .path
                            .get_ident()
                            .map(|i| i.to_string())
                            .as_deref()
                        {
                            Some(GETTER) => rules.gen_getter = parse_bool_or_str(&name_value.value),
                            Some(SETTER) => rules.gen_setter = parse_bool_or_str(&name_value.value),
                            Some(ALIAS) => {
                                if let Expr::Lit(lit) = &name_value.value {
                                    if let Lit::Str(x) = &lit.lit {
                                        rules.alias =
                                            Some(Ident::new(&x.value(), Span::call_site()));
                                    }
                                }
                            }
                            Some(PREFIX) => {
                                if let Expr::Lit(lit) = &name_value.value {
                                    if let Lit::Str(x) = &lit.lit {
                                        rules.prefix_setter = x.value();
                                    }
                                }
                            }
                            Some(INC_FOR_VEC) => {
                                if let Expr::Lit(lit) = &name_value.value {
                                    if let Lit::Bool(x) = &lit.lit {
                                        rules.inc_for_vec = x.value();
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Meta::Path(_) | Meta::List(_) => continue,
                }
            }
        }
    }

    rules
}
