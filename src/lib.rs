extern crate proc_macro;
use proc_macro::TokenStream;

use std::fs::File;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use std::io::Read;
use syn::parse::{ParseStream, Parse, Result};
use std::str::FromStr;

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);

    // let input = parse_macro_input!(input as DeriveInput);


    let tokens = quote! {
        // #input

        // struct Hello;
    };

    tokens.into()
}

struct FilenameInput {
    filename: String
}

impl Parse for FilenameInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit_file: syn::LitStr = input.parse()?;
        Ok(Self { filename: lit_file.value() })
    }
}

#[proc_macro]
pub fn my_macro_from_file(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FilenameInput);

    let cwd = std::env::current_dir().unwrap();
    let file_path = cwd.join(&input.filename);

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tokenized_content = proc_macro2::TokenStream::from_str(&contents).unwrap();

    // println!("{:#?}", tokenized_content);

    let tokens = quote!(
        rust_model_parser::my_macro!(#tokenized_content)
    );

    tokens.into()
}