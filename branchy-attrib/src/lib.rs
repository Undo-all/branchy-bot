extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn commandify(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as ItemFn);
    panic!("item = {:#?}", ast);
    attr
}
