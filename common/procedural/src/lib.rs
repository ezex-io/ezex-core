use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{self};
use syn::{DeriveInput, Lit, Meta, NestedMeta};

#[proc_macro_derive(Topic, attributes(topic_name))]
pub fn derive_topic(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ident = &ast.ident;
    match topic_name(ast.clone()) {
        Ok(topic_name) => {
            let gen = quote! {

                impl #ident {
                    pub const name: &'static str = #topic_name;
                }
                impl TopicMessage for #ident {
                    fn topic(&self) -> &'static str { #topic_name }
                    fn as_any(&self) -> &dyn std::any::Any { self }
                }
            };
            gen.into()
        }
        Err(msg) => {
            let tokens = quote_spanned! { proc_macro2::Span::call_site() =>
                compile_error!(#msg);
            };
            tokens.into()
        }
    }
}

pub(crate) fn topic_name(input: DeriveInput) -> Result<Lit, String> {
    let attrs = input.attrs;
    match attrs.get(0) {
        Some(attr) => match attr.parse_meta() {
            Ok(meta) => {
                let name = meta.path().get_ident();
                if name.expect("please add topic name") == "topic_name" {
                    match meta {
                        Meta::List(list) => match list.nested.first().unwrap() {
                            NestedMeta::Lit(l) => Ok(l.clone()),
                            _ => Err("Not a lit str".to_owned()),
                        },
                        _ => Err("Not a meta list".to_owned()),
                    }
                } else {
                    Err("invalid name, should be topic_name".to_owned())
                }
            }
            Err(_) => Err("Unable to parse meta".to_owned()),
        },
        None => Err("Need topic_name".to_owned()),
    }
}
