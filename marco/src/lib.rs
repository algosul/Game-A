#![feature(log_syntax)]
#![feature(proc_macro_diagnostic)]
use env_logger::Target::Stdout;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Data, DeriveInput, Fields, Meta};
#[proc_macro_derive(Scene, attributes(scene))]
pub fn scene_derive(input: TokenStream) -> TokenStream {
    env_logger::builder().target(Stdout).init();
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("#[derive(Scene)] Only structs for named fields are supported"),
        },
        _ => panic!("#[derive(Scene)] Only 'struct' are supported"),
    };
    let mut object_fields = Vec::new();
    for field in fields {
        for attr in &field.attrs {
            if !attr.path().is_ident("scene") {
                continue;
            }
            match &attr.style {
                AttrStyle::Outer => {
                    let meta = attr.parse_args::<Meta>().expect("Couldn't parse attribute");
                    match meta {
                        Meta::Path(path) => match path.get_ident() {
                            Some(ident) => match &*ident.to_string() {
                                "object" => {
                                    object_fields.push(field.ident.as_ref().unwrap().clone());
                                    continue;
                                }
                                _ => panic!("Unsupported attribute: {}", path.to_token_stream()),
                            },
                            None => panic!("Unsupported attribute: {}", path.to_token_stream()),
                        },
                        _ => panic!("Unsupported attribute: {}", attr.to_token_stream()),
                    }
                }
                _ => {
                    panic!("Unsupported attribute: {}", attr.to_token_stream());
                }
            }
        }
    }
    let expanded = quote! {
        impl Scene for #struct_name {
            fn get_objects(&self) -> Vec<&dyn Object> {
                vec![#( &self.#object_fields as &dyn Object ),*]
            }

            fn get_mut_objects(&mut self) -> Vec<&mut dyn Object> {
                vec![#( &mut self.#object_fields as &mut dyn Object ),*]
            }
        }
    };
    TokenStream::from(expanded)
}
