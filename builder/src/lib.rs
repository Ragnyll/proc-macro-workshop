use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Data, Fields, Field, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // these two will generate the same size vec as they are the same input tree
    let builder_struct = generate_builder_struct(&ast);
    let impl_builder = generate_impl_builder(&ast);

    let expanded = quote! {
        #builder_struct

        #impl_builder
    };

    TokenStream::from(expanded)
}

fn generate_builder_struct(syntax_tree: &DeriveInput) -> TokenStream2 {
    let input_ident = &syntax_tree.ident;
    let builder_struct_name = format!("{}Builder", input_ident.to_string());
    let builder_ident = Ident::new(&builder_struct_name, input_ident.span());

    let field_idents = generate_builder_named_fields_idents(&syntax_tree.data);
    let field_types = generate_builder_named_fields_types(&syntax_tree.data);

    quote! {
        pub struct #builder_ident {
            #(#field_idents: Option<#field_types>),*
        }

    }
}

fn generate_impl_builder(syntax_tree: &DeriveInput) -> TokenStream2 {
    let input_ident = &syntax_tree.ident;
    let builder_struct_name = format!("{}Builder", input_ident.to_string());
    let builder_ident = Ident::new(&builder_struct_name, input_ident.span());
    let field_idents = generate_builder_named_fields_idents(&syntax_tree.data);
    quote! {
        impl #input_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#field_idents: None),*
                }
            }
        }
    }
}

fn generate_builder_named_fields_idents(syntax_tree_data: &Data) -> Vec<&Ident> {
    let mut field_idents = vec![];
    let named_field_identifiers = get_named_fields(&syntax_tree_data).unwrap();
    for field in named_field_identifiers {
        // this feels spooky
        field_idents.push(field.ident.as_ref().unwrap());
    }
    field_idents
}

fn generate_builder_named_fields_types(syntax_tree_data: &Data) -> Vec<&Type> {
    let mut field_types = vec![];
    let named_field_identifiers = get_named_fields(&syntax_tree_data).unwrap();
    for field in named_field_identifiers {
        // this feels spooky
        field_types.push(&field.ty);
    }
    field_types
}

fn get_named_fields(
    syntax_tree_data: &Data,
) -> Option<&syn::punctuated::Punctuated<Field, syn::token::Comma>> {
    if let Data::Struct(data_struct) = syntax_tree_data {
        let fields = &data_struct.fields;
        if let Fields::Named(named_fields) = fields {
            return Some(&named_fields.named);
        }
    };
    None
}

