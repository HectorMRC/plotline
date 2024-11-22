use proc_macro::{TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn with_trigger(_meta: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}
