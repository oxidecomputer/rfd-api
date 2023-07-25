#![allow(dead_code, unused_imports)]

extern crate proc_macro;
#[macro_use]
extern crate quote;

use std::result::Iter;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Span, TokenTree};
use quote::ToTokens;
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    AttributeArgs, Block, DeriveInput, Ident, ItemFn, LitStr, Result, Token, Type,
};

#[proc_macro_attribute]
pub fn trace_request(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    let body_block = input.block;

    let wrapped_body_block: TokenStream = quote! {
        {
            use tracing::Instrument;

            fn get_status<T>(res: &Result<T, HttpError>) -> http::StatusCode where T: dropshot::HttpCodedResponse {
                match res {
                    Ok(_) => T::STATUS_CODE,
                    Err(err) => err.status_code,
                }
            }

            let request_id = &rqctx.request_id;
            let req = &rqctx.request;
            let uri = req.uri();
            let method = req.method();

            async {
                tracing::info!("Handling request");

                let start = std::time::Instant::now();
                let result = async #body_block.await;
                let end = std::time::Instant::now();

                let status = get_status(&result);
                let duration = end - start;

                match &result {
                    Ok(_) => tracing::info!(
                        ?status,
                        ?duration,
                        "Request complete"
                    ),
                    Err(err) => {
                        tracing::info!(
                            ?status,
                            ?duration,
                            external_message = ?err.external_message,
                            internal_message = ?err.internal_message,
                            "Request complete"
                        )
                    }
                };

                result
            }.instrument(tracing::info_span!("handler", ?request_id, ?method, ?uri)).await
        }
    }.into();

    input.block = Box::new(parse_macro_input!(wrapped_body_block as Block));

    quote! {
        #input
    }
    .into()
}
