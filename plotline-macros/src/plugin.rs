use plotline_plugin::kind::PluginKind;
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

/// The id of the plugin.
pub struct PluginIdArg {
    pub litstr: LitStr,
}

impl Parse for PluginIdArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::id>()?;

        let content;
        let _ = syn::parenthesized!(content in input);

        let litstr = content.parse::<LitStr>()?;
        Ok(Self { litstr })
    }
}

/// The kind of the plugin.
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
        let value = match ident.to_string().as_str() {
            "BeforeSaveExperience" => PluginKind::BeforeSaveExperience,
            other => return Err(input.error(format!("unknown plugin kind: {other}"))),
        };

        Ok(Self { ident, value })
    }
}

/// The version of the plugin.
pub struct PluginVersionArg {
    pub litstr: LitStr,
}

impl Parse for PluginVersionArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<keyword::version>()?;

        let content;
        let _ = syn::parenthesized!(content in input);

        let litstr = content.parse::<LitStr>()?;
        Ok(Self { litstr })
    }
}
