use proc_macro2::{Ident, Span};
use syn::{punctuated::Punctuated, Expr, Field, Lit, Meta, Token};

use crate::{
    ALIAS, ARGS, GETTER, GETTER_PREFIX, GETTER_PREFIX_DEFAULT, INC_FOR_VEC, SETTER, SETTER_PREFIX,
    SETTER_PREFIX_DEFAULT,
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
            prefix_setter: SETTER_PREFIX_DEFAULT.into(), // with, for all struct
            prefix_getter: GETTER_PREFIX_DEFAULT.into(), // nth, for unnamed struct
            gen_getter: true,
            gen_setter: true,
        }
    }
}

impl From<&Field> for Rules {
    fn from(field: &Field) -> Self {
        let mut rules = Rules::default();
        if let Some(attr) = &field.attrs.first() {
            if attr.path().is_ident(ARGS) {
                let nested =
                    match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
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
                                Some(GETTER) => {
                                    rules.gen_getter = Self::parse_bool_or_str(&name_value.value)
                                }
                                Some(SETTER) => {
                                    rules.gen_setter = Self::parse_bool_or_str(&name_value.value)
                                }
                                Some(ALIAS) => {
                                    if let Expr::Lit(lit) = &name_value.value {
                                        if let Lit::Str(x) = &lit.lit {
                                            rules.alias =
                                                Some(Ident::new(&x.value(), Span::call_site()));
                                        }
                                    }
                                }
                                Some(SETTER_PREFIX) => {
                                    if let Expr::Lit(lit) = &name_value.value {
                                        if let Lit::Str(x) = &lit.lit {
                                            rules.prefix_setter = x.value();
                                        }
                                    }
                                }
                                Some(GETTER_PREFIX) => {
                                    if let Expr::Lit(lit) = &name_value.value {
                                        if let Lit::Str(x) = &lit.lit {
                                            rules.prefix_getter = x.value();
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
            None => {
                // unnamed: index, alias
                match &self.alias {
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
                }
            }
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
