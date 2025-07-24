use crate::prelude::*;
use std::any::type_name;
use syn::{
    Attribute, Error, Expr, ExprLit, Field, Ident, Lit, Result, Token,
    meta::ParseNestedMeta,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

// TODO: migrate to new simpler attr.rs
#[derive(Default, Clone)]
pub struct MacroAttr {
    pub no_created_at: bool,
    pub no_updated_at: bool,
    pub no_deleted_at: bool,
    pub no_by_id: bool,
    /// model name in `#[crud]`
    pub model: String,
    /// to not use builtin generated inputs in `#[crud]`
    ///     use the inputs from the resolver instead
    pub resolver_inputs: bool,
    /// to not use builtin generated output in `#[crud]`
    ///     use the inputs from the resolver instead
    pub resolver_output: bool,
    /// to not generate db transaction `tx` in the resolver
    pub no_tx: bool,
}

impl Parse for MacroAttr {
    fn parse(s: ParseStream) -> Result<Self> {
        let mut a = MacroAttr::default();

        if s.peek(Ident) && !s.peek2(Token![=]) {
            a.model = str!(s.parse::<Ident>()?);
            if s.peek(Token![,]) {
                s.parse::<Token![,]>()?;
            }
        }

        while !s.is_empty() {
            let k = s.parse::<Ident>()?;
            s.parse::<Token![=]>()?;
            let v = s.parse::<Expr>()?;

            match str!(k).as_str() {
                "no_created_at" => a.no_created_at = expr_lit::<bool>(&v)?,
                "no_updated_at" => a.no_updated_at = expr_lit::<bool>(&v)?,
                "no_deleted_at" => a.no_deleted_at = expr_lit::<bool>(&v)?,
                "no_by_id" => a.no_by_id = expr_lit::<bool>(&v)?,
                "resolver_inputs" => a.resolver_inputs = expr_lit::<bool>(&v)?,
                "resolver_output" => a.resolver_output = expr_lit::<bool>(&v)?,
                "no_tx" => a.no_tx = expr_lit::<bool>(&v)?,
                other => {
                    return Err(Error::new(
                        k.span(),
                        format!(
                            "Attribute must be one of the MacroAttr fields, found {}",
                            other
                        ),
                    ));
                }
            }

            if s.peek(Token![,]) {
                s.parse::<Token![,]>()?;
            }
        }

        Ok(a)
    }
}

fn expr_lit<T: FromExprLit>(e: &Expr) -> syn::Result<T> {
    match T::expr_lit(e) {
        Some(v) => Ok(v),
        None => Err(Error::new(
            e.span(),
            format!(
                "Expr must be {} literal, found {}",
                type_name::<T>(),
                e.to_token_stream()
            ),
        )),
    }
}

trait FromExprLit: Sized {
    fn expr_lit(e: &Expr) -> Option<Self>;
}
impl FromExprLit for bool {
    fn expr_lit(e: &Expr) -> Option<Self> {
        match e {
            Expr::Lit(ExprLit {
                lit: Lit::Bool(b), ..
            }) => Some(b.value),
            _ => None,
        }
    }
}

fn eq_attr(a: &Attribute, attr: impl Display) -> bool {
    str!(a.path().to_token_stream()) == str!(attr)
}
fn eq_key(m: &ParseNestedMeta<'_>, key: impl Display) -> bool {
    m.path.get_ident().map(|i| str!(i)).unwrap_or_default() == str!(key)
}

pub fn has_attr(field: &Field, attr: impl Display) -> bool {
    for a in field.attrs.clone().into_iter() {
        if eq_attr(&a, &attr) {
            return true;
        }
    }
    false
}

pub fn has_attr_key(field: &Field, attr: impl Display, key: impl Display) -> bool {
    for a in field.attrs.clone().into_iter() {
        if eq_attr(&a, &attr) {
            let mut v = false;
            let _ = a.parse_nested_meta(|m| {
                if eq_key(&m, &key) {
                    v = true;
                }
                Ok(())
            });
            if v {
                return true;
            }
        }
    }
    false
}

pub fn get_attr(
    field: &Field,
    attr: impl Display,
    key: impl Display,
    default: impl Display,
) -> String {
    for a in field.attrs.clone().into_iter() {
        if eq_attr(&a, &attr) {
            let mut v = None;
            let _ = a.parse_nested_meta(|m| {
                if eq_key(&m, &key) && m.input.peek(syn::Token![=]) {
                    v = Some(str!(m.value()?));
                }
                Ok(())
            });
            if let Some(v) = v {
                return v;
            }
        }
    }
    str!(default)
}

pub fn must_get_attr(
    model: impl Display,
    field: &Field,
    attr: impl Display,
    key: impl Display,
    default: impl Display,
) -> String {
    let r = get_attr(&field, &attr, &key, &default);
    if r == "" {
        let field = field.ident.to_token_stream();
        panic!(
            "{}.{} attr={} key={} error: not found",
            model, field, attr, key
        );
    }
    r
}

pub trait MustGetAttr {
    fn impl_attr_model(&self) -> &dyn Display;
    fn impl_attr_field(&self) -> &Field;
    fn impl_attr_name(&self) -> &dyn Display;

    fn attr(&self, key: impl Display, default: impl Display) -> TokenStream2 {
        ts2!(self.attr_str(key, default))
    }
    fn attr_str(&self, key: impl Display, default: impl Display) -> String {
        let model = self.impl_attr_model();
        let field = self.impl_attr_field();
        let attr = self.impl_attr_name();
        must_get_attr(model, field, attr, key, default)
    }
    fn attr_panic(&self, err: impl Display) -> ! {
        let model = self.impl_attr_model();
        let field = self.impl_attr_field().ident.to_token_stream();
        let attr = self.impl_attr_name();
        panic!("{}.{} attr={} {}", model, field, attr, err);
    }
}
