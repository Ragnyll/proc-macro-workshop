use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Data, Fields, Field, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // these two will generate the same size vec as they are the same input tree
    let builder_struct = generate_builder_struct(&ast);
    let return_builder = generate_return_builder(&ast);
    let builder_impl = generate_builder_impl(&ast);

    let expanded = quote! {
        #builder_struct
        #return_builder
        #builder_impl
    };

    TokenStream::from(expanded)
}

fn generate_builder_struct(syntax_tree: &DeriveInput) -> TokenStream2 {
    let input_ident = &syntax_tree.ident;
    let builder_struct_name = format!("{}Builder", input_ident.to_string());
    let builder_ident = Ident::new(&builder_struct_name, input_ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = syntax_tree.data
    {
        named
    } else {
        unimplemented!();
    };

    //let fields = fields.iter().map(|f| {
        //let name = f.ident.as_ref().unwrap();
        //let t = &f.ty; // type

        //if is_optional_type(t) {
            //// you dont need the option on it since its already an option
            //quote! {
                //#name: #t
            //}
        //} else {
            //quote! {
                //#name: Option<#t>
            //}
        //}
    //});

    quote! {
        pub struct #builder_ident {
            #(#fields),*
        }

    }
}

fn is_optional_type(t: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = t {
        // this wont work for more complicated types
        for segment in type_path.path.segments.iter() {
            if segment.ident == "Option" {
                return true;
            }
        }
    }
    return false;
}

fn generate_return_builder(syntax_tree: &DeriveInput) -> TokenStream2 {
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

fn generate_builder_impl(syntax_tree: &DeriveInput) -> TokenStream2 {
    let input_ident = &syntax_tree.ident;
    let builder_struct_name = format!("{}Builder", input_ident.to_string());
    let builder_ident = Ident::new(&builder_struct_name, input_ident.span());
    let set_functions = generate_builder_set_functions(&syntax_tree);
    let build_function = generate_build_function(&syntax_tree);

    quote! {
        impl #builder_ident {
            #set_functions
            #build_function
        }
    }
}

fn generate_build_function(syntax_tree: &DeriveInput) -> TokenStream2 {
    let input_ident = &syntax_tree.ident;
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = syntax_tree.data
    {
        named
    } else {
        unimplemented!();
    };

    let checks = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();

        quote! {
            #name : self.#name.clone().ok_or("Field cannot be None")?
        }
    });

    quote! {
        pub fn build(&mut self) -> Result<#input_ident, Box<dyn std::error::Error>> {
            Ok(#input_ident {
                #(#checks),*
            })
        }
    }
}

fn generate_builder_set_functions(syntax_tree: &DeriveInput) -> TokenStream2 {
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = syntax_tree.data
    {
        named
    } else {
        unimplemented!();
    };

    let methods = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let ty = &f.ty;

        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    quote! {
        #(#methods)*
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
