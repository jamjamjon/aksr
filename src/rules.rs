use proc_macro2::{Ident, Span};
use syn::{Expr, ExprLit, Field, Lit};

use crate::{
    ALIAS, ALLOW, ARGS, EXCEPT, GETTER, GETTER_PREFIX, GETTER_PREFIX_DEFAULT, INC_FOR_VEC, SETTER,
    SETTER_PREFIX, SETTER_PREFIX_DEFAULT, SKIP,
};

#[derive(Debug)]
pub(crate) struct Rules {
    pub alias: Option<Ident>,
    pub inc_for_vec: bool,
    pub prefix_setter: String,
    pub prefix_getter: String,
    pub gen_getter: bool,
    pub gen_setter: bool,
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
                        Some(INC_FOR_VEC) => {
                            rules.inc_for_vec = meta
                                .value()
                                .map(|v| v.parse::<Expr>().map(|e| Rules::parse_bool_or_str(&e)))
                                .unwrap_or(Ok(true))
                                .unwrap_or(true);
                        }
                        Some(ALIAS) => {
                            let expr = meta.value()?.parse::<Expr>()?;
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(s), ..
                            }) = expr
                            {
                                rules.alias = Some(Ident::new(&s.value(), s.span()));
                            } else {
                                return Err(meta.error("Expected a string literal for ALIAS"));
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
                                    meta.error("Expected a string literal for SETTER_PREFIX")
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
                                    meta.error("Expected a string literal for GETTER_PREFIX")
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
                                        INC_FOR_VEC => rules.inc_for_vec = true,
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
                                        INC_FOR_VEC => rules.inc_for_vec = false,
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
                    panic!("Failed to parse attribute: {}", err);
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

    pub fn generate_setter_getter_names(&self, field: &Field, idx: usize) -> (Ident, Ident) {
        match &field.ident {
            None => match &self.alias {
                // unnamed: index, alias
                Some(alias) => {
                    let setter_name = Ident::new(
                        &format!("{}_{}", self.prefix_setter, alias),
                        Span::call_site(),
                    );
                    let getter_name = Ident::new(&format!("{}", alias), Span::call_site());
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
                    None => format!("{}", ident),
                    Some(alias) => format!("{}", alias),
                };
                let getter_name = Ident::new(&getter_name, Span::call_site());
                (setter_name, getter_name)
            }
        }
    }
}
