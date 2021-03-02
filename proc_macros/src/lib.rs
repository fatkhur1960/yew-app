extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use std::iter::FromIterator;
use syn::{parse_macro_input, DeriveInput, Lit};

#[derive(Default, Debug, Clone)]
struct VarAttribute {
    pub path: String,
    pub view: String,
}

#[proc_macro_attribute]
pub fn use_middleware(attrs: TokenStream, input: TokenStream) -> TokenStream {
    // let input_cloned = input.clone();
    // let input: DeriveInput = parse_macro_input!(input_cloned as DeriveInput);
    // let struct_name = &input.ident;

    // dbg!(&input);

    // if let syn::Data::Struct(_) = input.data {

    // } else {
    //     panic!("#[derive(Middleware)] can only be used with structs");
    // }
    
    // let ts = quote!{
    // };
    dbg!(&attrs);

    input
}

#[proc_macro_derive(RouteHolder, attributes(routes, views))]
pub fn route_parser(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut views: Vec<String> = Vec::new();
    let mut paths: Vec<String> = Vec::new();

    if let Some(attr) = input.attrs.into_iter().find(|a| a.path.is_ident("routes")) {
        for token in attr.tokens {
            if let TokenTree::Group(group) = token {
                for item in group.stream() {
                    if let TokenTree::Group(group) = item {
                        for inner in group.stream() {
                            if let TokenTree::Ident(ident) = &inner {
                                views.push(ident.to_string());
                            }
                            if let TokenTree::Literal(lit) = &inner {
                                if let Lit::Str(str) = Lit::new(lit.clone()) {
                                    paths.push(str.value());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let routes = paths
        .into_iter()
        .zip(views.into_iter())
        .collect::<Vec<(String, String)>>();

    let tts = {
        let mut arms: Vec<TokenStream2> = vec![];
        let mut middleware = vec![];
        for (path, view) in routes.into_iter() {
            let view = Ident::new(&view, Span::call_site());
            arms.push(quote! {
                #path => html!{ <views::#view params=params/> },
            });

            middleware.push(quote! {
                impl Middleware for views::#view {
                    fn before_enter(from: Route, to: Route, next: fn() -> Route) -> Route {
                        console_log!("from {} to {}", from.to_string(), to.to_string());
                        next()
                    }
                }
            });
        }
        let match_arms = TokenStream2::from_iter(arms.into_iter());

        let inner = quote! {
            use crate::middleware::Middleware;
            
            impl #name {
                pub fn new() -> Self {
                    let except: RouteMatcher = RouteMatcher::new("/", MatcherSettings::default()).unwrap();
                    let routes = ROUTES
                        .clone()
                        .into_iter()
                        .filter(|a| *a != except)
                        .collect();

                    #name {
                        routes,
                    }
                }

                pub fn render_view(&self, input: Route<bool>) -> Html {
                    let default = (input.to_string(), serde_json::Value::Null);
                    let (path, params) = if input.state {
                        default
                    } else {
                        match route_parser::parse(&input, self.routes.clone()) {
                            Some(result) => result,
                            None => default,
                        }
                    };

                    match path.as_ref() {
                        #match_arms
                        _ => html! {
                            <h1>{"Page not found"}</h1>
                        }
                    }
                }
            }
        };

        let mut output = TokenStream2::from_iter(middleware);
        output.extend(inner);
        output
    };

    TokenStream::from(tts)
}

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let fields: Vec<syn::Field> = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                panic!("#[derive(Model)] struct cannot have unnamed field");
            }
            fields.iter().cloned().collect()
        }
        _ => panic!("#[derive(Model)] can only be used with structs"),
    };

    let mut field_idents: Vec<syn::Ident> = vec![];
    let mut field_names: Vec<String> = vec![];
    let mut field_types: Vec<String> = vec![];

    for field in &fields {
        let field_ident = field.ident.clone().unwrap();
        let field_name = field_ident.to_string();
        let field_type = match field.ty {
            syn::Type::Path(syn::TypePath { ref path, .. }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                path.to_tokens(&mut tokens);
                tokens.to_string().replace(' ', "")
            }
            _ => panic!(
                "Type `{:?}` of field `{}` is not supported",
                field.ty, field_ident
            ),
        };

        field_idents.push(field_ident);
        field_names.push(field_name);
        field_types.push(field_type);
    }

    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let impl_ast = quote! {
        impl #impl_generics crate::components::forms::Model for #struct_name #ty_generics #where_clause {
            fn new() -> Self {
                Self {
                    ..Default::default()
                }
            }
        }

        impl #impl_generics crate::components::forms::FormValue for #struct_name #ty_generics #where_clause {
            fn fields(&self, prefix: &str, fields: &mut Vec<String>) {
                let field_prefix = if prefix == "" {
                    String::new()
                } else {
                    format!("{}.", prefix)
                };

                #(
                let field_path = format!("{}{}", field_prefix, #field_names);
                self.#field_idents.fields(&field_path, fields);
                )*
            }

            fn value(&self, field_path: &str) -> String {
                let (field_name, suffix) = crate::components::forms::split_field_path(field_path);

                match field_name {
                    #(
                    #field_names => self.#field_idents.value(suffix),
                    )*
                    _ => panic!(format!("Field {} does not exist in {}", field_path, stringify!(#struct_name)))
                }
            }

            fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
                let (field_name, suffix) = crate::components::forms::split_field_path(field_path);

                match field_name {
                    #(
                    #field_names => self.#field_idents.set_value(suffix, value),
                    )*
                    _ => panic!(format!("Field {} does not exist in {}", field_path, stringify!(#struct_name)))
                }
            }
        }
    };

    impl_ast.into()
}
