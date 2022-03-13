extern crate proc_macro;

use std::fmt::format;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self};

#[proc_macro_derive(Entity, attributes(column))]
pub fn entity_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input.clone()).unwrap();
    let input: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    let item_struct = syn::parse2::<syn::ItemStruct>(input).unwrap();

    // Build the trait implementation
    impl_entity_macro(ast, &item_struct)
}

fn impl_entity_macro(ast: syn::DeriveInput, item_struct: &syn::ItemStruct) -> TokenStream {
    let name = &ast.ident;
    println!("{:#?}", &ast);

    let struct_: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Usage of #[Modbus] on a non-struct type"),
    };

    for field in struct_.fields.iter() {
        for attr in field.attrs.iter() {
            let x: TokenStream = attr.tokens.clone().into();
            println!("{:#?}", x);
        }
    }

    // let fields = struct_.fields.iter().filter_map(|field| {
    //     for attr in field.attrs.iter() {
    //         let x: TokenStream = attr.tokens.clone().into();
    //         println!("{:#?}", x);
    //     }
    //     None
    // });

    let gen = quote! {
        impl Entity for #name {
            fn create_table() {
                println!("[Entity macro] create table {}", stringify!(#name).to_lowercase());
                // println!("{}", stringify!(#ty).to_lowercase());

            }
        }
    };
    gen.into()
}
