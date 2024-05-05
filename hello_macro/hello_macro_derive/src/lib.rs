use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input:TokenStream)->TokenStream{
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput)->TokenStream{
    let name = &ast.ident;
    let generation = quote! {
        impl HelloMacro for #name{
            fn hello_macro(){
                println!("Hey, Macro ! My name is {}", stringify!(#name));
            }
        }

    };
    generation.into()
}