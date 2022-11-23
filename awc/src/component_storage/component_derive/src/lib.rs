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
        let mut gen = quote!{
            use component_storage::ComponentStorage;
            use crate::table::TableID;
            use std::collections::HashMap;
        };

        let field_names = data_enum.variants.iter().map(|f| format_ident!("{}s", f.ident.to_string().to_case(Case::Snake)));
        let field_types = data_enum.variants.iter().map(|v| &v.ident);
        let struct_ts = quote!{
            pub struct Components{
                #( pub #field_names : ComponentStorage<EntityID, #field_types>), *,
                next_obj_id : EntityID,
                ids : HashMap<EntityID, ()>
            }
        };
        gen.extend(struct_ts);

        let field_names = data_enum.variants.iter().map(|f| format_ident!("{}s", f.ident.to_string().to_case(Case::Snake)));
        let new = quote!{
            pub fn new() -> Self{
                Self{
                    #( #field_names : ComponentStorage::new()), *,
                    next_obj_id : EntityID::new(0),
                    ids : HashMap::new(),
                }
            }
        };

        let field_names = data_enum.variants.iter().map(|f| format_ident!("{}s", f.ident.to_string().to_case(Case::Snake)));
        let field_types = data_enum.variants.iter().map(|v| &v.ident);
        let insert = quote!{
            pub fn insert(&mut self, entity : EntityID, component : Component){
                match component{
                    #( Component::#field_types(c) => {
                        self.#field_names.insert(entity, c);
                    }), *
                }
            }
        };

        let get_components_names = data_enum.variants.iter().map(|f| format_ident!("get_{}", f.ident.to_string().to_case(Case::Snake)));
        let field_names = data_enum.variants.iter().map(|f| format_ident!("{}s", f.ident.to_string().to_case(Case::Snake)));
        let field_types = data_enum.variants.iter().map(|v| &v.ident);
        let get_components = quote!{
            #( 
                pub fn #get_components_names(&self, entity : &EntityID) -> Option<&#field_types>{
                    self.#field_names.entry(entity)
                }
            )*
        };

        let get_components_names_mut = data_enum.variants.iter().map(|f| format_ident!("get_{}_mut", f.ident.to_string().to_case(Case::Snake)));
        let field_names = data_enum.variants.iter().map(|f| format_ident!("{}s", f.ident.to_string().to_case(Case::Snake)));
        let field_types = data_enum.variants.iter().map(|v| &v.ident);
        let get_components_mut = quote!{
            #( 
                pub fn #get_components_names_mut(&mut self, entity : &EntityID) -> Option<&mut #field_types>{
                    self.#field_names.entry_mut(entity)
                }
            )*
        };

        let components_ts = quote!{
            impl Components{
                
                #new

                #insert

                #get_components

                #get_components_mut

                pub fn alloc_id(&mut self) -> EntityID{
                    let id = self.next_obj_id;
                    self.next_obj_id = id.next();
                    self.ids.insert(id, ());
                    id
                }

                pub fn ids(&self) -> Vec<EntityID>{
                    self.ids.keys().copied().collect()
                }
            }
        };
        gen.extend(components_ts);

        let mut news = quote!{};
        for v in data_enum.variants.iter(){
            let new_fun = format_ident!("new_{}", v.ident.to_string().to_case(Case::Snake));
            let struct_type =  &v.ident;
            
            let quote = quote!{
                #[macro_export]
                macro_rules! #new_fun {
                    ($($args:expr),*) => {                        
                        use awc::component::Component;

                        awc::component::#struct_type::new($($args),*)
                    }
                }
            };
            news.extend(quote);
        }

        gen.extend(news);
        gen.into()
    } 
    else {
        return Error::new(name.span(), "Only implemented for enums")
        .to_compile_error()
        .into();
    }

    
}