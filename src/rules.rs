#![allow(deprecated)]

use proc_macro2::{Ident, Span};
use syn::{Expr, ExprLit, Field, Lit};

use crate::{
    ALIAS, ALIAS_DEPRECATED, ALLOW, ARGS, EXCEPT, EXTEND, EXTEND_DEPRECATED, GETTER, GETTER_PREFIX,
    GETTER_PREFIX_DEFAULT, GETTER_VISIBILITY, SETTER, SETTER_PREFIX, SETTER_PREFIX_DEFAULT,
    SETTER_VISIBILITY, SKIP,
};

#[derive(Debug)]
pub(crate) struct Rules {
    pub alias: Option<Ident>,
    pub inc_for_vec: bool,
    pub prefix_setter: String,
    pub prefix_getter: String,
    pub gen_getter: bool,
    pub gen_setter: bool,
    pub getter_visibility: Option<String>,
    pub setter_visibility: Option<String>,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            alias: None,
            inc_for_vec: false,
            prefix_setter: SETTER_PREFIX_DEFAULT.into(),
            prefix_getter: GETTER_PREFIX_DEFAULT.into(),
            gen_getter: true,
            gen_setter: true,
            getter_visibility: None, // Default: pub
            setter_visibility: None, // Default: pub
        }
    }
}

impl From<&Field> for Rules {
    fn from(field: &Field) -> Self {
        let mut rules = Rules::default();

        if let Some(attr) = field.attrs.first() {
            if attr.path().is_ident(ARGS) {
                if let Err(err) = attr.parse_nested_meta(|meta| {
                    match meta.path.get_ident().map(|i| i.to_string()).as_deref() {
                        Some(GETTER) => {
                            rules.gen_getter = meta
                                .value()
                                .map(|v| v.parse::<Expr>().map(|e| Rules::parse_bool_or_str(&e)))
                                .unwrap_or(Ok(true))
                                .unwrap_or(true);
                        }
                        Some(SETTER) => {
                            rules.gen_setter = meta
                                .value()
                                .map(|v| v.parse::<Expr>().map(|e| Rules::parse_bool_or_str(&e)))
                                .unwrap_or(Ok(true))
                                .unwrap_or(true);
                        }
                        Some(SKIP) => {
                            let skip = meta
                                .value()
                                .map(|v| v.parse::<Expr>().map(|e| Rules::parse_bool_or_str(&e)))
                                .unwrap_or(Ok(true))
                                .unwrap_or(true);
                            rules.gen_getter = !skip;
                            rules.gen_setter = !skip;
                        }
                        Some(EXTEND) | Some(EXTEND_DEPRECATED) => {
                            rules.inc_for_vec = meta
                                .value()
                                .map(|v| v.parse::<Expr>().map(|e| Rules::parse_bool_or_str(&e)))
                                .unwrap_or(Ok(true))
                                .unwrap_or(true);
                        }
                        Some(ALIAS) | Some(ALIAS_DEPRECATED) => {
                            let expr = meta.value()?.parse::<Expr>()?;
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            }) = expr
                            {
                                rules.alias = Some(Ident::new(&s.value(), s.span()));
                            } else {
                                return Err(meta.error("Expected a string literal for alias"));
                            }
                        }
                        Some(SETTER_PREFIX) => {
                            if let Ok(Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            })) = meta.value().and_then(|v| v.parse::<Expr>())
                            {
                                rules.prefix_setter = s.value();
                            } else {
                                return Err(
                                    meta.error("Expected a string literal for setter_prefix")
                                );
                            }
                        }
                        Some(GETTER_PREFIX) => {
                            if let Ok(Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            })) = meta.value().and_then(|v| v.parse::<Expr>())
                            {
                                rules.prefix_getter = s.value();
                            } else {
                                return Err(
                                    meta.error("Expected a string literal for getter_prefix")
                                );
                            }
                        }
                        Some(GETTER_VISIBILITY) => {
                            if let Ok(Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            })) = meta.value().and_then(|v| v.parse::<Expr>())
                            {
                                let vis = s.value();
                                rules.getter_visibility = Some(Rules::parse_visibility(&vis));
                            } else {
                                return Err(
                                    meta.error("Expected a string literal for getter_visibility")
                                );
                            }
                        }
                        Some(SETTER_VISIBILITY) => {
                            if let Ok(Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            })) = meta.value().and_then(|v| v.parse::<Expr>())
                            {
                                let vis = s.value();
                                rules.setter_visibility = Some(Rules::parse_visibility(&vis));
                            } else {
                                return Err(
                                    meta.error("Expected a string literal for setter_visibility")
                                );
                            }
                        }
                        Some(ALLOW) => {
                            meta.parse_nested_meta(|nested| {
                                if let Some(ident) = nested.path.get_ident() {
                                    match ident.to_string().as_str() {
                                        GETTER => rules.gen_getter = true,
                                        SETTER => rules.gen_setter = true,
                                        SKIP => {
                                            rules.gen_getter = false;
                                            rules.gen_setter = false;
                                        }
                                        EXTEND | EXTEND_DEPRECATED => rules.inc_for_vec = true,
                                        _ => return Err(nested.error("Unsupported allow argument")),
                                    }
                                }
                                Ok(())
                            })?;
                        }
                        Some(EXCEPT) => {
                            meta.parse_nested_meta(|nested| {
                                if let Some(ident) = nested.path.get_ident() {
                                    match ident.to_string().as_str() {
                                        GETTER => rules.gen_getter = false,
                                        SETTER => rules.gen_setter = false,
                                        SKIP => {
                                            rules.gen_getter = true;
                                            rules.gen_setter = true;
                                        }
                                        EXTEND | EXTEND_DEPRECATED => rules.inc_for_vec = false,
                                        _ => {
                                            return Err(nested.error("Unsupported except argument"))
                                        }
                                    }
                                }
                                Ok(())
                            })?;
                        }
                        _ => return Err(meta.error("Unsupported argument")),
                    }
                    Ok(())
                }) {
                    panic!("Failed to parse attribute: {err}");
                }
            }
        }

        rules
    }
}

impl Rules {
    pub fn parse_bool_or_str(value: &Expr) -> bool {
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

    /// Parse visibility keyword to full visibility string
    /// Supports: "pub", "public", "private", "pub(crate)", "pub(self)", "pub(super)"
    pub fn parse_visibility(vis: &str) -> String {
        match vis.to_lowercase().as_str() {
            "pub" | "public" => "pub".to_string(),
            "private" => "".to_string(), // Empty means private
            "crate" => "pub(crate)".to_string(),
            "self" => "pub(self)".to_string(),
            "super" => "pub(super)".to_string(),
            _ if vis.starts_with("pub(") => vis.to_string(), // pub(crate), pub(self), pub(super), pub(in path)
            _ => vis.to_string(),                            // Preserve as-is for full syntax
        }
    }

    /// Generates visibility tokens for getter methods
    pub fn getter_visibility_token(&self) -> proc_macro2::TokenStream {
        Rules::visibility_token_impl(&self.getter_visibility)
    }

    /// Generates visibility tokens for setter methods
    pub fn setter_visibility_token(&self) -> proc_macro2::TokenStream {
        Rules::visibility_token_impl(&self.setter_visibility)
    }

    /// Internal implementation for generating visibility tokens
    fn visibility_token_impl(vis_option: &Option<String>) -> proc_macro2::TokenStream {
        use proc_macro2::TokenStream;
        use quote::quote;

        match vis_option {
            Some(vis) if vis.is_empty() => {
                // Private (no pub)
                TokenStream::new()
            }
            Some(vis) => {
                // Parse the visibility string to generate tokens
                match vis.as_str() {
                    "pub" => quote! { pub },
                    "pub(crate)" => quote! { pub(crate) },
                    "pub(self)" => quote! { pub(self) },
                    "pub(super)" => quote! { pub(super) },
                    vis if vis.starts_with("pub(in ") => {
                        // Handle pub(in path::to::module)
                        let path_str = vis.strip_prefix("pub(in ").unwrap();
                        let path_str = path_str.strip_suffix(')').unwrap_or(path_str);
                        let path: syn::Path = syn::parse_str(path_str)
                            .unwrap_or_else(|_| panic!("Invalid visibility path: {vis}"));
                        quote! { pub(in #path) }
                    }
                    _ => quote! { pub }, // Default to pub
                }
            }
            None => {
                // Default to pub
                quote! { pub }
            }
        }
    }

    pub fn generate_setter_getter_names(&self, field: &Field, idx: usize) -> (Ident, Ident) {
        match &field.ident {
            None => match &self.alias {
                // unnamed: index, alias
                Some(alias) => {
                    let setter_name = Ident::new(
                        &format!("{}_{}", self.prefix_setter, alias),
                        Span::call_site(),
                    );
                    let getter_name = Ident::new(&format!("{alias}"), Span::call_site());
                    (setter_name, getter_name)
                }
                None => {
                    let setter_name = Ident::new(
                        &format!("{}_{}", self.prefix_setter, idx),
                        Span::call_site(),
                    );
                    let getter_name = Ident::new(
                        &format!("{}_{}", self.prefix_getter, idx),
                        Span::call_site(),
                    );
                    (setter_name, getter_name)
                }
            },
            Some(ident) => {
                // named: ident, alias
                let setter_name = match &self.alias {
                    None => format!("{}_{}", self.prefix_setter, ident),
                    Some(alias) => format!("{}_{}", self.prefix_setter, alias),
                };
                let setter_name = Ident::new(&setter_name, Span::call_site());

                let getter_name = match &self.alias {
                    None => format!("{ident}"),
                    Some(alias) => format!("{alias}"),
                };
                let getter_name = Ident::new(&getter_name, Span::call_site());
                (setter_name, getter_name)
            }
        }
    }
}
