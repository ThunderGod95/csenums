use proc_macro::TokenStream;

use proc_macro_error::{abort, proc_macro_error};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro]
#[proc_macro_error]
pub fn csenum(item: TokenStream) -> TokenStream {
    let cs = parse_macro_input!(item as DeriveInput);

    let cs_name = &cs.ident;

    match &cs_name {
        _ => abort!("Only enums are supported.")
    }
}