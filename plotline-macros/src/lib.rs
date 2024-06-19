mod plugin;
use plugin::*;

use plotline_plugin::kind::PluginKind;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn plugin(args: TokenStream, input: TokenStream) -> TokenStream {
    let PluginArgs { id, kind, version } = parse_macro_input!(args as PluginArgs);

    let input_fn = parse_macro_input!(input as ItemFn);
    let input_ident = &input_fn.sig.ident;

    let plugin_closure = match kind.value {
        PluginKind::BeforeSaveExperience => quote! {
            |input: plotline_proto::plugin::BeforeSaveExperienceInput| {
                let subject = plotline_plugin::proto::into_experience(&input.subject).unwrap();
                let timeline = input.timeline.iter()
                    .map(|experience| {
                        plotline_plugin::proto::into_experience(experience).unwrap()
                    })
                    .collect::<Vec<_>>();

                match #input_ident(&subject, &timeline) {
                    Ok(()) => Default::default(),
                    Err(err) => plotline_proto::plugin::BeforeSaveExperienceOutput {
                        error: protobuf::MessageField::some(
                            plotline_plugin::proto::from_error(&err)
                        ),
                        ..Default::default()
                    }
                }
            }
        },
    };

    let plugin_output = quote! {
        let output_bytes = output.write_to_bytes().unwrap();
        let output_len = (output_bytes.len() as u32).to_le_bytes();
        [&output_len[..], &output_bytes].concat()
    };

    let plugin_id = id.litstr;
    let plugin_kind = kind.ident;
    let plugin_version = version.litstr;

    TokenStream::from(quote! {
        #[no_mangle]
        fn id() -> *const u8 {
            let output = {
                // block is needed to make sure scoped data is released before
                // returning. Otherwise wasm heap may overflow.

                use protobuf::Message;

                let output = plotline_proto::plugin::GetPluginId {
                    id: #plugin_id.into(),
                    ..Default::default()
                };

                #plugin_output
            };

            output.as_ptr()
        }

        #[no_mangle]
        fn kind() -> *const u8 {
            let output = {
                // block is needed to make sure scoped data is released before
                // returning. Otherwise wasm heap may overflow.

                use protobuf::{EnumOrUnknown, Message};

                let output = plotline_proto::plugin::GetPluginKind {
                    kind: EnumOrUnknown::new(#plugin_kind.into()),
                    ..Default::default()
                };

                #plugin_output
            };

            output.as_ptr()
        }

        #[no_mangle]
        fn version() -> *const u8 {
            let output = {
                // block is needed to make sure scoped data is released before
                // returning. Otherwise wasm heap may overflow.

                use protobuf::{EnumOrUnknown, Message};

                let output = plotline_proto::plugin::GetPluginVersion {
                    version: #plugin_version.into(),
                    ..Default::default()
                };

                #plugin_output
            };

            output.as_ptr()
        }

        #[no_mangle]
        fn run(ptr: u32) -> *const u8 {
            let output = {
                // block is needed to make sure scoped data is released before
                // returning. Otherwise wasm heap may overflow.
                
                use protobuf::Message;

                let input = unsafe {
                    let len = *(ptr as *const u32);
                    let bytes = (ptr + 4) as *const u8;
                    let slice = core::slice::from_raw_parts(bytes, len as usize);
                    Message::parse_from_bytes(slice).unwrap()
                };

                #input_fn

                let call = #plugin_closure;
                let output = call(input);

                #plugin_output
            };

            output.as_ptr()
        }
    })
}
