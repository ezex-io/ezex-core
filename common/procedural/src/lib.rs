use proc_macro::TokenStream;
use quote::{
    quote,
    quote_spanned,
};
use syn::{
    self,
    DeriveInput,
    Lit,
    Meta,
    NestedMeta,
    parse_macro_input,
};

#[proc_macro_derive(Topic, attributes(topic_name))]
pub fn derive_topic(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ident = &ast.ident;
    match topic_name(ast.clone()) {
        Ok(topic_name) => {
            let gen_code = quote! {

                impl #ident {
                    pub const name: &'static str = #topic_name;
                }
                impl TopicMessage for #ident {
                    fn topic(&self) -> &'static str { #topic_name }
                    fn as_any(&self) -> &dyn std::any::Any { self }
                }
            };
            gen_code.into()
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
    match attrs.first() {
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

/// custom derive macro to generate prefix for env vars
#[proc_macro_derive(EnvPrefix, attributes(prefix))]
pub fn derive_env_prefix(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // println!("Debug: All attributes: {:?}", ast.attrs);

    let prefix = ast
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("prefix"))
        .and_then(|attr| match attr.parse_meta() {
            Ok(meta) => match meta {
                Meta::NameValue(name_value) => {
                    if let Lit::Str(s) = name_value.lit {
                        Some(s.value())
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Err(_) => None,
        })
        .unwrap_or_else(|| "EZEX".to_string());

    let name = &ast.ident;

    let r#gen = quote! {
        impl #name {
            pub fn prepend_envs() {
                use std::env;
                use clap::Args;

                let cmd = <Self as Args>::augment_args(clap::Command::new(""));
                for arg in cmd.get_arguments() {
                    if let Some(env_var) = arg.get_env() {
                        if let Some(env_name) = env_var.to_str() {
                            // get the original value
                            if let Ok(value) = env::var(env_name) {
                                // create prefixed var name
                                let prefixed_var = format!("{}_{}", #prefix, env_name);
                                println!("Debug: Setting {} = {}", prefixed_var, value);
                                // set the new env var
                                unsafe {
                                    env::set_var(&prefixed_var, &value);
                                }
                                println!("Debug: Verifying {} = {:?}", prefixed_var, env::var(&prefixed_var));
                            }
                        }
                    }
                }
            }
        }
    };

    r#gen.into()
}
