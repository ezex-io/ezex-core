use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{self, DeriveInput, Expr, Lit, parse_macro_input};

#[proc_macro_derive(Event, attributes(event_key))]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ident = &ast.ident;
    match event_key(ast.clone()) {
        Ok(event_key) => {
            let gen_code = quote! {
                impl #ident {
                    pub const name: &'static str = #event_key;
                }
                impl EventMessage for #ident {
                    fn key(&self) -> String { #event_key.to_string() }
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

/// handle both attr formats:
/// - #[event_key("foo:bar")]
/// - #[event_key = "foo:bar"]
pub(crate) fn event_key(input: DeriveInput) -> Result<Lit, String> {
    let attrs = input.attrs;

    // Find the event_key attribute
    match attrs.iter().find(|attr| attr.path().is_ident("event_key")) {
        Some(attr) => {
            // Try to parse as a string literal directly
            match attr.parse_args::<Lit>() {
                Ok(lit) => Ok(lit),
                Err(_) => {
                    // If direct parsing fails, try the more complex approach
                    let meta = attr.meta.clone();
                    if let Ok(list) = meta.require_list() {
                        if let Ok(lit) = list.parse_args::<Lit>() {
                            return Ok(lit);
                        }
                    }

                    Err(
                        "Expected a literal string for event_key, e.g., #[event_key(\"my:event\")]"
                            .to_owned(),
                    )
                }
            }
        }
        None => Err("Need event_key attribute".to_owned()),
    }
}

/// custom derive macro to generate prefix for env vars
#[proc_macro_derive(EnvPrefix, attributes(env_prefix))]
pub fn derive_env_prefix(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // println!("Debug: All attributes: {:?}", ast.attrs);

    let prefix = ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("env_prefix"))
        .and_then(|attr| {
            if let Ok(meta) = attr.meta.require_name_value() {
                // In syn 2.0, meta.value is an Expr, not a Lit
                if let Expr::Lit(expr_lit) = &meta.value {
                    if let Lit::Str(s) = &expr_lit.lit {
                        Some(s.value())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or_default();

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
                                // set the new env var
                                unsafe {
                                    env::set_var(&prefixed_var, &value);
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    r#gen.into()
}
