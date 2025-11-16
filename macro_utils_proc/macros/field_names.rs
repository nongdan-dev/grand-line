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
            pan!("{name} struct should be named fields");
        }
    } {
        let attrs = Attr::from_field(&s!(name), &f, &|_| false);
        if let Some(a) = attrs.iter().find(|a| a.is("field_names")) {
            f.attrs = attrs
                .iter()
                .filter(|b| b.attr != a.attr)
                .map(|a| a.field_attr())
                .collect();
            let a = a.clone().into_with_validate::<FieldNamesAttr>();
            if a.virt {
                if s!(f.to_token_stream()).starts_with("pub ") {
                    let err = a.inner.err("virtual field name should not be public");
                    pan!("{err}");
                }
                if s!(f.ty.to_token_stream()) != "!" {
                    let err = a.inner.err("virtual field name type should be `!`");
                    pan!("{err}");
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
        let f_str = s!(f);
        all.push(quote! {
            #f_str,
        });
        let f = scream!("F", f);
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
        ["skip", "virt"].map(|f| s!(f)).to_vec()
    }
}
