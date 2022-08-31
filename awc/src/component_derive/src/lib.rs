use proc_macro::{TokenStream};
use quote::{quote, format_ident};
use syn::{self, Data, Error};

#[proc_macro_derive(ComponentCollection)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_component_macro(&ast)
}

fn impl_component_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    if let Data::Enum(data_enum) = data 
    {
        let mut gen = quote! {

        };
        for variant in &data_enum.variants{
            let ref variant_name = variant.ident;
            let mut is_variant_func_name = format_ident!("is_{}", variant_name.to_string());
            is_variant_func_name.set_span(variant_name.span());

            let ts = quote! {
                pub fn #is_variant_func_name() {
                    println!("Hello, Macro! My name is {}!", stringify!(#name));
                }
            };
            gen.extend(ts.into_iter());
        }

        gen.into()
    } 
    else {
        return Error::new(name.span(), "Only implemented for enums")
        .to_compile_error()
        .into();
    }

    
}