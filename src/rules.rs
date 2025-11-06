#![allow(deprecated)]

use proc_macro2::{Ident, Span};
use syn::{Expr, ExprLit, Field, Lit};

use crate::{
    ALIAS, ALIAS_DEPRECATED, ALLOW, ARGS, EXCEPT, EXTEND, EXTEND_DEPRECATED, GETTER, GETTER_INLINE,
    GETTER_PREFIX, GETTER_PREFIX_DEFAULT, GETTER_VISIBILITY, INLINE, INTO_PREFIX, SETTER,
    SETTER_INLINE, SETTER_PREFIX, SETTER_PREFIX_DEFAULT, SETTER_VISIBILITY, SKIP, VISIBILITY,
};

#[derive(Debug)]
pub(crate) struct Rules {
    pub alias: Option<Ident>,
    pub inc_for_vec: bool,
    pub prefix_setter: String,
    pub prefix_getter: String,
    pub gen_getter: bool,
    pub gen_setter: bool,
    pub gen_into: bool,
    pub into_prefix: Option<String>,
    pub getter_visibility: Option<String>,
    pub setter_visibility: Option<String>,
    pub getter_inline: Option<InlineMode>,
    pub setter_inline: Option<InlineMode>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum InlineMode {
    None,    // No inline attribute
    Default, // #[inline]
    Always,  // #[inline(always)]
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            alias: None,
            inc_for_vec: false,
            prefix_setter: SETTER_PREFIX_DEFAULT.into(),
            prefix_getter: String::new(), // Empty for named structs, will use "nth" for tuple structs
            gen_getter: true,
            gen_setter: true,
            gen_into: true,
            into_prefix: None,                        // Default: "into"
            getter_visibility: None,                  // Default: pub
            setter_visibility: None,                  // Default: pub
            getter_inline: Some(InlineMode::Always),  // Default: #[inline(always)] for getters
            setter_inline: Some(InlineMode::Default), // Default: #[inline] for setters
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
                                let value = s.value();
                                // setter_prefix cannot be empty, use default if empty
                                rules.prefix_setter = if value.is_empty() {
                                    SETTER_PREFIX_DEFAULT.into()
                                } else {
                                    value
                                };
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
                        Some(INTO_PREFIX) => {
                            if let Ok(Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            })) = meta.value().and_then(|v| v.parse::<Expr>())
                            {
                                rules.into_prefix = Some(s.value());
                            } else {
                                return Err(meta.error("Expected a string literal for into_prefix"));
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
                        Some(INLINE) => {
                            // Parse inline for both getter and setter
                            let inline_mode = Rules::parse_inline_value(&meta)?;
                            rules.getter_inline = Some(inline_mode);
                            rules.setter_inline = Some(inline_mode);
                        }
                        Some(GETTER_INLINE) => {
                            rules.getter_inline = Some(Rules::parse_inline_value(&meta)?);
                        }
                        Some(SETTER_INLINE) => {
                            rules.setter_inline = Some(Rules::parse_inline_value(&meta)?);
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
                                        "into" => rules.gen_into = false,
                                        _ => {
                                            return Err(nested.error("Unsupported except argument"))
                                        }
                                    }
                                }
                                Ok(())
                            })?;
                        }
                        Some(VISIBILITY) => {
                            if let Ok(Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            })) = meta.value().and_then(|v| v.parse::<Expr>())
                            {
                                let vis = s.value();
                                let vis_val = Rules::parse_visibility(&vis);
                                rules.getter_visibility = Some(vis_val.clone());
                                rules.setter_visibility = Some(vis_val);
                            } else {
                                return Err(meta.error("Expected a string literal for visibility"));
                            }
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

    /// Parse inline attribute value
    /// Supports: #[args(inline)], #[args(inline = true)], #[args(inline = "always")]
    pub fn parse_inline_value(meta: &syn::meta::ParseNestedMeta) -> syn::Result<InlineMode> {
        // Try to parse value
        if let Ok(value) = meta.value() {
            let expr = value.parse::<Expr>()?;
            match expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Bool(b), ..
                }) => {
                    if b.value {
                        Ok(InlineMode::Default)
                    } else {
                        Ok(InlineMode::None)
                    }
                }
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => {
                    match s.value().to_lowercase().as_str() {
                        "always" => Ok(InlineMode::Always),
                        "true" | "yes" | "default" => Ok(InlineMode::Default),
                        "false" | "no" | "none" => Ok(InlineMode::None),
                        _ => Err(meta
                            .error("Expected 'always', 'true', 'false', or no value for inline")),
                    }
                }
                _ => Err(meta.error("Expected a boolean or string for inline")),
            }
        } else {
            // No value means #[args(inline)] -> default inline
            Ok(InlineMode::Default)
        }
    }

    /// Generate inline attribute tokens for getter
    pub fn getter_inline_token(&self) -> proc_macro2::TokenStream {
        Rules::inline_token_impl(&self.getter_inline)
    }

    /// Generate inline attribute tokens for setter
    pub fn setter_inline_token(&self) -> proc_macro2::TokenStream {
        Rules::inline_token_impl(&self.setter_inline)
    }

    /// Internal implementation for generating inline tokens
    fn inline_token_impl(inline_option: &Option<InlineMode>) -> proc_macro2::TokenStream {
        use proc_macro2::TokenStream;
        use quote::quote;

        match inline_option {
            Some(InlineMode::Always) => quote! { #[inline(always)] },
            Some(InlineMode::Default) => quote! { #[inline] },
            Some(InlineMode::None) | None => TokenStream::new(),
        }
    }

    pub fn generate_setter_getter_names(&self, field: &Field, idx: usize) -> (Ident, Ident) {
        match &field.ident {
            None => {
                // Tuple struct: for getter, if prefix is empty and no alias, use "nth" as default
                let actual_getter_prefix = if self.prefix_getter.is_empty() && self.alias.is_none()
                {
                    GETTER_PREFIX_DEFAULT
                } else {
                    &self.prefix_getter
                };

                match &self.alias {
                    // Tuple struct with alias
                    Some(alias) => {
                        // setter_prefix is never empty (enforced in parsing)
                        let setter_name = format!("{}_{}", self.prefix_setter, alias);
                        let setter_name = Ident::new(&setter_name, Span::call_site());

                        // getter: if prefix is empty, use alias directly; otherwise prefix_alias
                        let getter_name = if actual_getter_prefix.is_empty() {
                            format!("{alias}")
                        } else {
                            format!("{actual_getter_prefix}_{alias}")
                        };
                        let getter_name = Ident::new(&getter_name, Span::call_site());
                        (setter_name, getter_name)
                    }
                    None => {
                        // Tuple struct without alias: use index
                        // setter_prefix is never empty (enforced in parsing)
                        let setter_name = format!("{}_{}", self.prefix_setter, idx);
                        let setter_name = Ident::new(&setter_name, Span::call_site());

                        // getter: use actual_getter_prefix (which defaults to "nth" for tuple structs)
                        let getter_name = format!("{actual_getter_prefix}_{idx}");
                        let getter_name = Ident::new(&getter_name, Span::call_site());
                        (setter_name, getter_name)
                    }
                }
            }
            Some(ident) => {
                // Named struct
                let name_or_alias = self.alias.as_ref().unwrap_or(ident);

                // setter: always use prefix (prefix is never empty)
                let setter_name = format!("{}_{}", self.prefix_setter, name_or_alias);
                let setter_name = Ident::new(&setter_name, Span::call_site());

                // getter: if prefix is empty, use name/alias directly; otherwise prefix_name
                let getter_name = if self.prefix_getter.is_empty() {
                    format!("{name_or_alias}")
                } else {
                    format!("{}_{}", self.prefix_getter, name_or_alias)
                };
                let getter_name = Ident::new(&getter_name, Span::call_site());
                (setter_name, getter_name)
            }
        }
    }
}
