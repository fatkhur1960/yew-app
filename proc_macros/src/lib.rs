extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenTree};
use std::iter::FromIterator;
use syn::{parse_macro_input, DeriveInput, Lit};

#[derive(Debug, Clone)]
struct VarAttribute {
    pub path: String,
    pub view: String,
}

#[proc_macro_derive(RouteHolder, attributes(to))]
pub fn route_holder(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ref name = input.ident;

    let enum_: syn::DataEnum = match input.data.clone() {
        syn::Data::Enum(data) => data,
        _ => panic!("Usage of #[MyRouter] on a non-enum type"),
    };

    let mut variant_checker_functions = TokenStream2::new();
    let mut var_attrs: Vec<VarAttribute> = Vec::new();

    for (index, variant) in enum_.variants.iter().enumerate() {
        variant_checker_functions = TokenStream2::new();

        // let _fields_in_variant = match &variant.fields {
        //     Fields::Unnamed(_) => quote_spanned! { variant.ident.span() => (..) },
        //     Fields::Unit => quote_spanned! { variant.ident.span() => },
        //     Fields::Named(_) => quote_spanned! { variant.ident.span() => {..} },
        // };

        let mut var_attr: VarAttribute;

        for attr in variant.attrs.iter() {
            if attr.path.is_ident("to") {
                let args = proc_macro2::TokenStream::from(attr.tokens.clone());
                let items = args.into_iter();

                let mut path = String::new();
                let mut view = String::new();

                for item in items {
                    if let TokenTree::Group(group) = item {
                        for inner in group.stream() {
                            if let TokenTree::Group(g) = inner {
                                for i in g.stream() {
                                    if let TokenTree::Ident(ident) = &i {
                                        view = ident.to_string();
                                    }
                                    if let TokenTree::Literal(lit) = &i {
                                        if let Lit::Str(str) = Lit::new(lit.clone()) {
                                            path = str.value();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                var_attr = VarAttribute { path, view };
                var_attrs.push(var_attr.clone());
            }
        }

        let attr = var_attrs[index].clone();
        let view = Ident::new(&attr.view, Span::call_site());

        variant_checker_functions.extend(quote_spanned! { variant.ident.span() =>
            fn get_view(&self, params: JsonValue) -> Html {
                use crate::views;

                html! {
                    <views::#view params=params/>
                }
            }
        });
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut expanded = quote! {
        use crate::utils::route_parser::parse_get_key;
        use crate::{JsonValue, utils::RouteHandler};

        impl #impl_generics #name #ty_generics #where_clause {
            #variant_checker_functions
        }
    };

    let tts = {
        let mut sas = vec![];
        for var_attr in var_attrs.iter() {
            let path = var_attr.path.clone();
            sas.push(quote! {
                paths.push(#path.to_string());
            });
        }
        let sases = TokenStream2::from_iter(sas.into_iter());

        let mut arms: Vec<TokenStream2> = vec![];
        for attr in var_attrs.iter() {
            let path = attr.path.clone();
            let view = Ident::new(&attr.view, Span::call_site());
            arms.push(quote! {
                #path => Some(#name::#view),
            })
        }
        let match_arms = TokenStream2::from_iter(arms.into_iter());

        quote! {
            impl #name {
                fn get_paths() -> Vec<String> {
                    let mut paths: Vec<String> = Vec::new();
                    #sases
                    paths
                }
            }

            impl RouteHandler {
                pub fn new() -> Self {
                    RouteHandler{}
                }

                pub fn render_view(&self, input: String) -> Html {
                    if let Some((path, params)) = parse_get_key(#name::get_paths(), input) {
                        let result = match path.as_ref() {
                            #match_arms
                            _ => None
                        };
                        if let Some(result) = result {
                            result.get_view(params)
                        } else {
                            html! {
                                <h1>{"Page not found"}</h1>
                            }
                        }
                    } else {
                        html! {
                            <h1>{"No route registered"}</h1>
                        }
                    }
                }
            }
        }
    };

    expanded.extend(tts);
    TokenStream::from(expanded)
}
