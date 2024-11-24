mod trigger;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

/// Generates the trigger API for a struct.
#[proc_macro_attribute]
pub fn with_trigger(_: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_item = parse_macro_input!(item as ItemStruct);
    trigger::impl_with_trigger(parsed_item)
        .unwrap_or_else(|error| error.into_compile_error().into())
}
