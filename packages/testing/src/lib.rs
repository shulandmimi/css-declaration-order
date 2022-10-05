use std::{env, path::PathBuf};

use glob::glob;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens, __private::parse};
use syn::{parse_macro_input, token::Token, AttributeArgs, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn testing(
    attr: proc_macro::TokenStream,
    token: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let call = parse_macro_input!(token as ItemFn);
    let call_name = call.sig.ident.clone();
    let attr = parse_macro_input!(attr as AttributeArgs);

    let path = attr
        .iter()
        .find_map(|item| {
            if let NestedMeta::Lit(Lit::Str(s)) = item {
                return Some(s.value());
            }
            None
        })
        .expect("attr need is a &str");

    let base_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect(
        "#[fixture] requires CARGO_MANIFEST_DIR because it's relative to cargo manifest directory",
    ));

    let dir = base_dir.join(path.as_str());
    let dir = dir.to_str().unwrap();

    let matchs = glob(dir).expect("direction is not exists");

    let matchs = matchs
        .filter_map(|item| {
            if let Ok(path) = item {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    let mut tokens: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

    matchs.into_iter().for_each(|filename| {
        let name = Ident::new(
            filename[base_dir.to_string_lossy().len()..filename.len() - 4]
                .replace("/", "__")
                .as_str(),
            Span::call_site(),
        );

        let f = quote! {

            #[test]
            fn #name() {
                #call_name(PathBuf::from(#filename));
            }

        };

        let item = syn::parse2::<ItemFn>(f).expect("test_case should parsed");

        item.to_tokens(&mut tokens);
    });

    let block = quote! {
        #call

        #tokens
    };

    block.into()
}

// fn expand() -> TokenStream {}
