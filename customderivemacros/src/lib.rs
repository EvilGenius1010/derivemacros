extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input,DeriveInput,Data};
mod se;

#[proc_macro_derive(SerializeNovice)]
pub fn serialize_item_novice(item:TokenStream)->TokenStream{

    let ast = parse_macro_input!(item as DeriveInput);

    if !matches!(ast.data,Data::Struct(_)){
        return quote! {
            compile_error!("MyMacro can only be applied to structs, not enums or unions.")
        }.into()
    } 
    else{
        let name = &ast.ident;
        return quote! {
            impl #name {
            pub fn hello_from_macro(&self) {
                // Now we are inside a function body, so println! is valid here.
                // println!("Hello from the SerializeNovice macro on struct {}!", stringify!(#name));


            }
        }
        }.into()
    }
}

// #[proc_macro_derive(SerializeIntermediate)]
// pub fn serialize_item_intermediate(item:TokenStream)->TokenStream{
        
// }