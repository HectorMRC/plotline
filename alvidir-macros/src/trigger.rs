use std::borrow::BorrowMut;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser, spanned::Spanned, Error, Fields, GenericParam, Ident, ItemStruct, TypeParam,
};

pub fn impl_with_trigger(mut parsed_item: ItemStruct) -> syn::Result<TokenStream> {
    let original_generic_params = parsed_item.generics.params.clone();

    let span = parsed_item.span();
    let before = Ident::new("B", span);
    parsed_item
        .generics
        .params
        .borrow_mut()
        .push(GenericParam::Type(TypeParam::from(before.clone())));

    let after = Ident::new("A", span);
    parsed_item
        .generics
        .params
        .borrow_mut()
        .push(GenericParam::Type(TypeParam::from(after.clone())));

    let Fields::Named(fields) = parsed_item.fields.borrow_mut() else {
        return Err(Error::new_spanned(
            parsed_item,
            "struct with unamed fields is not valid",
        ));
    };

    let original_fields: Vec<_> = fields
        .named
        .iter()
        .map(|field| {
            let field_name = field.ident.clone();
            quote! {
                #field_name: self.inner.#field_name,
            }
        })
        .collect();

    fields.named.push(syn::Field::parse_named.parse2(quote! {
        /// The command to execute before performing the transaction.
        ///
        /// If this command fails the whole transaction is aborted.
        pub before: B
    })?);

    fields.named.push(syn::Field::parse_named.parse2(quote! {
        /// The command to execute once the transaction is completed.
        ///
        /// If this command fails the transaction IS NOT rollbacked, but the resulting error is retrived as the transaction's result.
        pub after: A
    })?);

    let name = &parsed_item.ident;
    let generic_params = &parsed_item.generics.params;
    let where_clause = &parsed_item.generics.where_clause;

    Ok(quote! {
        #parsed_item

        impl<#generic_params> #name<#generic_params>
        #where_clause
        {
            /// Configure triggers for this transaction.
            pub fn with_trigger(self) -> WithTrigger<Self> {
                self.into()
            }
        }

        impl<#generic_params> WithTrigger<#name<#generic_params>>
        #where_clause
        {
            /// Schedules the given command to be executed before the transaction is performed.
            pub fn before<C>(self, command: C) -> #name<#original_generic_params, LiFoChain<C, #before>, #after> {
                #name {
                    #(#original_fields)*
                    after: self.inner.after,
                    before: LiFoChain {
                        head: self.inner.before,
                        value: command,
                    },
                }
            }

            /// Schedules the given command to be executed after the transaction is completed.
            pub fn after<C>(self, command: C) -> #name<#original_generic_params, #before, LiFoChain<C, #after>> {
                #name {
                    #(#original_fields)*
                    before: self.inner.before,
                    after: LiFoChain {
                        head: self.inner.after,
                        value: command,
                    },
                }
            }
        }
    }
    .into())
}
