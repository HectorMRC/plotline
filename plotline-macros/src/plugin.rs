use plotline_plugin::kind::PluginKind;
use std::str::FromStr;
use strum::VariantNames;
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr
};

mod keyword {
    syn::custom_keyword!(kind);
    syn::custom_keyword!(id);
}

pub struct PluginArgs {
    /// The id of the plugin.
    pub id: PluginIdArg,
    /// The kind of the plugin.
    pub kind: PluginKindArg,
}

impl Parse for PluginArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut id: Option<PluginIdArg> = None;
        let mut kind: Option<PluginKindArg> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keyword::kind) {
                if kind.is_some() {
                    return Err(input.error("the plugin kind must be set once"));
                }

                kind = Some(input.parse()?);
            } else if lookahead.peek(keyword::id) {
                if id.is_some() {
                    return Err(input.error("the plugin id must be set once"));
                }

                let mut plugin_id: PluginIdArg = input.parse()?;
                plugin_id.value = plugin_id.value.trim().to_string();

                if plugin_id.value.is_empty() {
                    return Err(input.error("the plugin id cannot be empty"));
                }

                id = Some(plugin_id);
            } else {
                // Parse the unrecognized token tree to advance the parse
                // stream, and throw it away so we can keep parsing. Otherwise
                // this would become an endless while loop.
                let _ = input.parse::<proc_macro2::TokenTree>();
            }
        }

        Ok(PluginArgs {
            id: id.ok_or(input.error("the plugin id must be set once"))?,
            kind: kind.ok_or(input.error("the plugin kind must be set once"))?,
        })
    }
}

pub struct PluginKindArg{
    pub ident: Ident,
    pub value: PluginKind
}

impl Parse for PluginKindArg {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        input.parse::<keyword::kind>()?;

        let content;
        let _ = syn::parenthesized!(content in input);

        let ident = content.parse::<Ident>()?;
        let Ok(value) = PluginKind::from_str(&ident.to_string()) else {
            return Err(input.error(format!(
                "kind must be one of: {}",
                PluginKind::VARIANTS.join(", ")
            )));
        };

        Ok(Self{
            ident,
            value
        })
    }
}

pub struct PluginIdArg{
    pub value: String
}

impl Parse for PluginIdArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::id>()?;

        let content;
        let _ = syn::parenthesized!(content in input);

        Ok(Self{value: content.parse::<LitStr>()?.value()})
    }
}
