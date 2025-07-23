use crate::*;
use strum_macros::{Display, EnumString};
use syn::Field;

pub struct Relation {
    pub ty: RelationTy,
    pub f: Field,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Display, EnumString)]
pub enum RelationTy {
    #[strum(serialize = "belongs_to")]
    BelongsTo,
    #[strum(serialize = "has_one")]
    HasOne,
    #[strum(serialize = "has_many")]
    HasMany,
    #[strum(serialize = "many_to_many")]
    ManyToMany,
}

impl Relation {
    pub fn model(&self) -> TokenStream2 {
        self.f.ty.to_token_stream()
    }
    pub fn gql(&self) -> TokenStream2 {
        ty_gql(self.model())
    }
    pub fn name(&self) -> TokenStream2 {
        self.f.ident.to_token_stream()
    }
    pub fn gql_name(&self) -> String {
        camel_str!(self.name())
    }
    pub fn sql_dep(&self) -> String {
        match self.ty {
            RelationTy::BelongsTo => snake_str!(self.name(), "id"),
            RelationTy::HasOne => str!("id"),
            RelationTy::HasMany => str!("id"),
            _ => panic!("TODO:"),
        }
    }
    pub fn id(&self) -> TokenStream2 {
        match self.ty {
            RelationTy::BelongsTo => snake!(self.name(), "id"),
            RelationTy::HasOne => ts2!("id"),
            RelationTy::HasMany => ts2!("id"),
            _ => panic!("TODO:"),
        }
    }
    pub fn column(&self) -> TokenStream2 {
        match self.ty {
            RelationTy::BelongsTo => pascal!("id"),
            RelationTy::HasOne => pascal!(extract_attr_value(&self.f, "has_one", "fkey").unwrap()),
            RelationTy::HasMany => {
                pascal!(extract_attr_value(&self.f, "has_many", "fkey").unwrap())
            }
            _ => panic!("TODO:"),
        }
    }
}
use syn::Lit;
use syn::Result;

pub fn extract_attr_value(f: &Field, attr_name: &str, key_name: &str) -> Result<String> {
    let mut found = str!("");
    for attr in f.attrs.clone().into_iter() {
        if str!(attr.path().to_token_stream()) == attr_name {
            attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    if ident == key_name && meta.input.peek(syn::Token![=]) {
                        let lit: Lit = meta.value()?.parse()?;
                        found = match lit {
                            Lit::Str(s) => s.value(),
                            _ => return Err(meta.error("unsupported literal type")),
                        };
                    }
                }
                Ok(())
            })?;
        }
    }
    Ok(found)
}

pub fn is_attr(f: &Field, attr_name: &str) -> bool {
    for attr in f.attrs.clone().into_iter() {
        if str!(attr.path().to_token_stream()) == attr_name {
            return true;
        }
    }
    false
}
