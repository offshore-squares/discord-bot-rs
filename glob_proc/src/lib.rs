use glob;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Token, Type};

/// default_export: Command
struct GlobPattern {
    export: Ident,
    r#type: Type,
}

impl Parse for GlobPattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let export: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let r#type: Type = input.parse()?;

        Ok(Self { export, r#type })
    }
}

#[proc_macro]
pub fn glob_use(input: TokenStream) -> TokenStream {
    let GlobPattern { export, r#type } = parse_macro_input!(input as GlobPattern);

    let files = glob::glob("*.rs").unwrap();

    let mut filenames = vec![];
    for file in files {
        let file = file.unwrap();
        let filename = file.to_str().unwrap().to_owned();
        let filename = filename.trim_end_matches(".rs").to_string();
        filenames.push(filename);
    }

    let expanded = quote! {
        #(mod #filenames;)*

        pub fn #export() -> Vec<#r#type> {
            let mut exports = vec![];

            #(exports.push(&mut #filenames::#export);)*

            return exports;
        }
    };

    TokenStream::from(expanded)
}
