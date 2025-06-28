#![feature(log_syntax)]
#![feature(proc_macro_diagnostic)]
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Data, DeriveInput, Fields, Meta};
#[proc_macro_derive(Scene, attributes(scene))]
pub fn scene_derive(input: TokenStream) -> TokenStream {
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
    let mut objects_field = None;
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
                                    if objects_field.is_some() {
                                        panic!(
                                            "Only one of the attributes 'objects' and 'object' is \
                                             allowed"
                                        )
                                    }
                                    object_fields.push(field.ident.as_ref().unwrap().clone());
                                    continue;
                                }
                                "objects" => {
                                    if objects_field
                                        .replace(field.ident.as_ref().unwrap().clone())
                                        .is_some()
                                    {
                                        panic!("Attribute 'objects' only allow one");
                                    }
                                    if !object_fields.is_empty() {
                                        panic!(
                                            "Only one of the attributes 'objects' and 'object' is \
                                             allowed"
                                        )
                                    }
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
    let expanded = match (object_fields, objects_field) {
        (object_fields, None) => quote! {
            impl Scene for #struct_name {
                fn get_objects<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn Object> + 'a> {
                    Box::new(vec![#( &self.#object_fields as &dyn Object ),*].into_iter())
                }

                fn get_mut_objects<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut (dyn Object + 'static)> + 'a> {
                    Box::new(vec![#( &mut self.#object_fields as &mut dyn Object ),*].into_iter())
                }
            }

        },
        (x, objects_field) if x.is_empty() => quote! {
            impl Scene for #struct_name {
                fn get_objects<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn Object> + 'a> {
                    Box::new(self.#objects_field.iter().map(|x| x.as_ref()))
                }

                fn get_mut_objects<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut (dyn Object + 'static)> + 'a> {
                    Box::new(self.#objects_field.iter_mut().map(|x| x.as_mut()))
                }
            }

        },
        (_, _) => {
            panic!("Only one of the attributes 'objects' and 'object' is allowed");
        }
    };
    TokenStream::from(expanded)
}
