use convert_case::{Casing, Case};
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
        let field_types = data_enum.variants.iter().map(|f| &f.ident);
        let field_names = data_enum.variants.iter().map(|f| format_ident!("{}s", f.ident.to_string().to_case(Case::Snake)));
        let field_names_c = field_names.clone();
        let gen = quote!{
            use component_storage::ComponentStorage;
            pub struct Components{
                #( pub #field_names : ComponentStorage<i32, #field_types>), *
            }

            impl Components{
                pub fn new() -> Self{
                    Self{
                        #( #field_names_c : ComponentStorage::new()), *
                    }
                }
            }
        };

        gen.into()
    } 
    else {
        return Error::new(name.span(), "Only implemented for enums")
        .to_compile_error()
        .into();
    }

    
}