use crate::prelude::*;

pub static TY_INCLUDE_DELETED: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert(s!(MacroTy::Search));
    set.insert(s!(MacroTy::Count));
    set.insert(s!(MacroTy::Detail));
    set
});

pub fn push_include_deleted(inputs: &Ts2) -> Ts2 {
    quote! {
        #inputs
        include_deleted: Option<bool>,
    }
}
