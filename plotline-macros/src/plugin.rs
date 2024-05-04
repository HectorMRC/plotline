use plotline_plugin::{PluginId, PluginKind, PluginVersion};
use std::str::FromStr;
use strum::VariantNames;
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr,
};

mod keyword {
    syn::custom_keyword!(id);
    syn::custom_keyword!(kind);
    syn::custom_keyword!(version);
}

pub struct PluginArgs {
    /// The id of the plugin.
    pub id: PluginIdArg,
    /// The kind of the plugin.
    pub kind: PluginKindArg,
    /// The version of the plugin.
    pub version: PluginVersionArg,
}

impl Parse for PluginArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut id: Option<PluginIdArg> = None;
        let mut kind: Option<PluginKindArg> = None;
        let mut version: Option<PluginVersionArg> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keyword::id) {
                if id.is_some() {
                    return Err(input.error("the plugin id must be set once"));
                }

                id = Some(input.parse()?);
            } else if lookahead.peek(keyword::kind) {
                if kind.is_some() {
                    return Err(input.error("the plugin kind must be set once"));
                }

                kind = Some(input.parse()?);
            } else if lookahead.peek(keyword::version) {
                if version.is_some() {
                    return Err(input.error("the plugin version must be set once"));
                }

                version = Some(input.parse()?);
            } else {
                // Parse the unrecognized token tree to advance the parse
                // stream, and throw it away so we can keep parsing. Otherwise
                // this would become an endless while loop.
                let _ = input.parse::<proc_macro2::TokenTree>();
            }
        }

        Ok(PluginArgs {
            id: id.ok_or(input.error("the plugin id must be set"))?,
            kind: kind.ok_or(input.error("the plugin kind must be set"))?,
            version: version.ok_or(input.error("the plugin version must be set"))?,
        })
    }
}
pub struct PluginIdArg {
    pub value: PluginId,
}

impl Parse for PluginIdArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::id>()?;

        let content;
        let _ = syn::parenthesized!(content in input);

        PluginId::from_str(&content.parse::<LitStr>()?.value())
            .map_err(|err| input.error(err))
            .map(|value| Self { value })
    }
}

pub struct PluginKindArg {
    pub ident: Ident,
    pub value: PluginKind,
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

        Ok(Self { ident, value })
    }
}

pub struct PluginVersionArg {
    pub value: PluginVersion,
}

impl Parse for PluginVersionArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::version>()?;

        let content;
        let _ = syn::parenthesized!(content in input);

        PluginVersion::from_str(&content.parse::<LitStr>()?.value())
            .map_err(|err| input.error(err))
            .map(|value| Self { value })
    }
}
