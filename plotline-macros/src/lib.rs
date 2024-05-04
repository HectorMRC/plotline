mod plugin;
use plugin::*;

use plotline_plugin::kind::PluginKind;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    let PluginArgs { id, kind } = parse_macro_input!(args as PluginArgs);
    
    let input_fn = parse_macro_input!(input as ItemFn);
    let input_ident = &input_fn.sig.ident;

    let (input, output) = match kind.value {
        PluginKind::BeforeSaveExperience => (
            quote!(BeforeSaveExperienceInput),
            quote!(BeforeSaveExperienceOutput),
        ),
    };

    let plugin_id = id.value;
    let plugin_kind = kind.ident;

    TokenStream::from(quote! {
        #[no_mangle]
        fn id() -> *const u8 {
            let output = GetPluginId {
                id: #plugin_id.into(),
                ..Default::default()
            };

            let output_bytes = output.write_to_bytes().unwrap();
            let output_len = (output_bytes.len() as u32).to_le_bytes();
            let output_bytes = [&output_len[..], &output_bytes].concat();
            output_bytes.as_ptr()
        }

        #[no_mangle]
        fn kind() -> *const u8 {
            let output = GetPluginKind {
                kind: EnumOrUnknown::new(#plugin_kind.into()),
                ..Default::default()
            };

            let output_bytes = output.write_to_bytes().unwrap();
            let output_len = (output_bytes.len() as u32).to_le_bytes();
            let output_bytes = [&output_len[..], &output_bytes].concat();
            output_bytes.as_ptr()
        }

        #[no_mangle]
        fn run(ptr: u32) -> *const u8 {
            let input = unsafe {
                let len = *(ptr as *const u32);
                let bytes = (ptr + 4) as *const u8;
                let slice = core::slice::from_raw_parts(bytes, len as usize);
                #input::parse_from_bytes(slice).unwrap()
            };

            let output: #output = match #input_ident(input) {
                Ok(_) => Default::default(),
                Err(err) => err.into(),
            };

            let output_bytes = output.write_to_bytes().unwrap();
            let output_len = (output_bytes.len() as u32).to_le_bytes();
            let output_bytes = [&output_len[..], &output_bytes].concat();
            output_bytes.as_ptr()
        }

        #input_fn
    })
}
