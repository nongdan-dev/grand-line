use crate::prelude::*;

pub static TY_INCLUDE_DELETED: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert(MacroTy::Search.to_string());
    set.insert(MacroTy::Count.to_string());
    set.insert(MacroTy::Detail.to_string());
    set
});

pub fn push_include_deleted(inputs: Ts2, include_deleted: bool) -> Ts2 {
    if include_deleted {
        quote! {
            #inputs
            include_deleted: Option<bool>,
        }
    } else {
        inputs
    }
}
pub fn get_include_deleted(include_deleted: bool) -> Ts2 {
    if include_deleted {
        quote!(include_deleted)
    } else {
        quote!(None)
    }
}
