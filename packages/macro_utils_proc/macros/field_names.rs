use crate::prelude::*;
use core::panic;

pub fn gen_field_names(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);

    let name = item.ident.to_token_stream();
    let mut fields = vec![];
    let mut idents = vec![];

    for mut f in match item.fields {
        Fields::Named(f) => f.named,
        _ => {
            panic!("{name} struct should be named fields");
        }
    } {
        let attrs = Attr::from_field(&name.to_string(), &f, &|_| false);
        if let Some(a) = attrs.iter().find(|a| a.is("field_names")) {
            f.attrs = attrs
                .iter()
                .filter(|b| b.attr != a.attr)
                .map(|a| a.field_attr())
                .collect();
            let a = a.clone().into_with_validate::<FieldNamesAttr>();
            if a.virt {
                if f.to_token_stream().to_string().starts_with("pub ") {
                    a.inner.panic("virtual field should not be public");
                }
                if f.ty.to_token_stream().to_string() != "!" {
                    a.inner.panic("virtual field type should be `!`");
                }
            }
            if !a.skip {
                idents.push(f.ident.to_token_stream())
            }
            if !a.virt {
                fields.push(f);
            }
        } else {
            idents.push(f.ident.to_token_stream());
            fields.push(f);
        }
    }

    item.fields = Fields::Named(FieldsNamed {
        named: Punctuated::from_iter(fields),
        brace_token: Default::default(),
    });

    let mut all = vec![];
    let mut impls = vec![];
    for f in idents {
        let f_str = f.to_string();
        all.push(quote! {
            #f_str,
        });
        let f = format!("FIELD_{f}").to_shouty_snake_case().ts2_or_panic();
        impls.push(quote! {
            pub const #f: &'static str = #f_str;
        });
    }
    let l = all.len();

    quote! {
        #item
        impl #name {
            pub const F: [&'static str; #l] = [#(#all)*];
            #(#impls)*
        }
    }
    .into()
}

struct FieldNamesAttr {
    skip: bool,
    virt: bool,
    pub inner: Attr,
}
impl From<Attr> for FieldNamesAttr {
    fn from(a: Attr) -> Self {
        Self {
            skip: a.bool("skip").unwrap_or(false),
            virt: a.bool("virt").unwrap_or(false),
            inner: a,
        }
    }
}
impl AttrValidate for FieldNamesAttr {
    fn attr_fields(_: &Attr) -> Vec<String> {
        ["skip", "virt"].map(|f| f.to_owned()).to_vec()
    }
}
