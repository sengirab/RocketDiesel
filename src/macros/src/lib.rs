#![feature(extern_crate_item_prelude)]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(CRD, attributes(table_name, model))]
pub fn crd(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the implementation
    let gen = impl_crd(&ast);
    gen.parse().unwrap()
}

fn impl_crd(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let attr = &ast.attrs;

    for item in attr {
        println!("{:?}", item.value.name());
    }

    quote! {
        impl #name {
            pub fn create() {
                println!("Yay, i work {:?}", stringify!(#name));
            }
        }
    }
}
