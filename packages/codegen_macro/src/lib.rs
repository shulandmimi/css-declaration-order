use quote::quote;

use syn::{Block, FnArg, ImplItemMethod, Type, TypeReference};

#[proc_macro_attribute]
pub fn emitter(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let method: ImplItemMethod = syn::parse(item).expect("faile to parse input as an item");

    let item = expand(method);

    quote!( #item ).into()
}

fn expand(i: ImplItemMethod) -> ImplItemMethod {
    let method_name = i.sig.ident.clone();

    assert!(
        method_name.to_string().starts_with("emit_"),
        "emitter need emit_prefix but got {}",
        method_name
    );

    let node_type = i
        .sig
        .inputs
        .clone()
        .into_iter()
        .nth(1)
        .and_then(|arg| match arg {
            FnArg::Typed(ty) => Some(ty.ty),
            _ => None,
        })
        .map(|ty| match *ty {
            Type::Reference(TypeReference { elem, .. }) => *elem,
            _ => panic!("Type of node parameter should be reference"),
        })
        .expect("#[emitter] methods should have signature of fn(&mut self, node: Node) -> Result");

    let block = i.block;

    let item = quote! {
        {
            impl<W, S> crate::Emit<#node_type> for crate::CodeGenerator<W, S> where W: crate::Writer, S: crate::SepSerialize<SepRule> + crate::SepSerialize<FormatSep> {
                fn emit(&mut self, node: &#node_type) -> crate::Result {
                    self.#method_name(node)
                }
            }

            #block

            #[allow(unreachable_code)]
            {
                return Ok(());
            }
        }
    };

    let debug_item = item.clone();

    let new_block = syn::parse2::<Block>(item).unwrap_or_else(|err| {
        panic!(
            "{}\n>>>>>>>>>>>>\n{}",
            err.to_string(),
            debug_item.to_string()
        );
    });
    ImplItemMethod {
        block: new_block,
        ..i
    }
}
