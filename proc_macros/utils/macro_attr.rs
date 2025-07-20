use quote::ToTokens;
use std::any::type_name;
use syn::{
    Error, Expr, ExprLit, Ident, Lit, Result, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

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
            a.model = s.parse::<Ident>()?.to_string();
            if s.peek(Token![,]) {
                s.parse::<Token![,]>()?;
            }
        }

        while !s.is_empty() {
            let k = s.parse::<Ident>()?;
            s.parse::<Token![=]>()?;
            let v = s.parse::<Expr>()?;

            match k.to_string().as_str() {
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
