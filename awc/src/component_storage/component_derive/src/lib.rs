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
        let field_types = data_enum.variants.iter().map(|v| &v.ident);
        let field_types_gc = field_types.clone();
        let field_types_ac = field_types.clone();

        let get_components_names = data_enum.variants.iter().map(|f| format_ident!("get_{}", f.ident.to_string().to_case(Case::Snake)));

        let field_names = data_enum.variants.iter().map(|f| format_ident!("{}s", f.ident.to_string().to_case(Case::Snake)));
        let field_names_c = field_names.clone();
        let field_names_gc = field_names.clone();
        let field_names_ac = field_names.clone();

        let gen = quote!{
            use component_storage::ComponentStorage;
            use crate::table::TableID;

            pub struct Components{
                #( pub #field_names : ComponentStorage<EntityID, #field_types>), *,
                next_obj_id : EntityID,
            }

            impl Components{
                pub fn new() -> Self{
                    Self{
                        #( #field_names_c : ComponentStorage::new()), *,
                        next_obj_id : EntityID::new(0),
                    }
                }

                pub fn insert(&mut self, entity : EntityID, component : Component){
                    match component{
                        #( Component::#field_types_ac(c) => {
                            self.#field_names_ac.insert(entity, c);
                        }), *
                    }
                }

                #( 
                    pub fn #get_components_names(&mut self, entity : &EntityID) -> Option<&mut #field_types_gc>{
                        self.#field_names_gc.entry(entity)
                    }
                )*

                /* Altenartively, could declare an ComponentType enum. Take that as param, then use match expresion to extract the correct one. */

                pub fn alloc_id(&mut self) -> EntityID{
                    let id = self.next_obj_id;
                    self.next_obj_id = id.next();
                    id
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