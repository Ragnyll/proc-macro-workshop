use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Data, DataStruct, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let input_ident = ast.ident;
    let builder_struct_name = format!("{}Builder", input_ident.to_string());
    let builder_ident = Ident::new(&builder_struct_name, input_ident.span());

    // these will always be the same order and size
    let mut field_idents = vec![];
    let mut field_types = vec![];
    if let Data::Struct(data_struct) = ast.data {
        let fields = data_struct.fields;
        if let Fields::Named(named_fields) = fields {
            let named_field_identifiers = named_fields.named;
            for field in named_field_identifiers {
                field_idents.push(field.ident);
                field_types.push(field.ty);
            }
        }
    };

    let expanded = quote!{
        pub struct #builder_ident {
            #(#field_idents: Option<#field_types>),*
        }
        impl #input_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#field_idents: None),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
