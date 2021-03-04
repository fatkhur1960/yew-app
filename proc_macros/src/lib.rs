extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::ToTokens;
use std::iter::FromIterator;
use syn::{parse_macro_input, DataEnum, DeriveInput};

#[proc_macro_derive(Query)]
pub fn parse_queryst(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        syn::Data::Struct(_) => (),
        _ => panic!("#[derive(Query)] can only be used with structs"),
    }

    let output = quote! {
        use yew_router::prelude::RouteService;
        use crate::{utils::route_parser, JsonValue};

        impl #name {
            pub fn new() -> Self {
                let query: #name = serde_json::from_value(Self::parsed_query()).unwrap();

                query
            }

            /// get current query string
            fn parsed_query() -> JsonValue {
                let rs: RouteService<bool> = RouteService::new();
                let raw_qs = rs.get_query();

                route_parser::parse_queryst(&raw_qs)
            }
        }
    };

    output.into()
}

#[proc_macro_derive(Params)]
pub fn parse_params(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        syn::Data::Struct(_) => (),
        _ => panic!("#[derive(Params)] can only be used with structs"),
    }

    let output = quote! {
        use yew_router::prelude::RouteService;
        use crate::utils::route_parser;
        use crate::routes::AppRouteHandler;
        use crate::JsonValue;

        impl #name {
            pub fn new() -> Self {
                let param: #name = serde_json::from_value(Self::parsed_params()).unwrap();
                param
            }

            /// get current path params
            fn parsed_params() -> JsonValue {
                let rs: RouteService<bool> = RouteService::new();
                let rh = AppRouteHandler::new();
                let path = rs.get_route().to_string();

                match route_parser::parse(&path, rh.get_routes()) {
                    Some((_, params)) => params,
                    None => serde_json::Value::Null,
                }
            }
        }
    };

    output.into()
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

#[derive(Debug)]
struct RouteMeta {
    pub name: String,
    pub path: String,
    pub view: String,
    pub auth: bool,
    pub navbar: bool,
    pub sidebar: bool,
}

#[proc_macro_derive(RouteDerive, attributes(params))]
pub fn derive_route(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let mut routes: Vec<RouteMeta> = vec![];
    let mut variant_checker_functions = TokenStream2::new();

    match input.data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            for var in variants.into_iter() {
                let ident = var.ident;
                let (mut path, mut view, mut auth, mut navbar, mut sidebar) =
                    (String::new(), String::new(), false, true, true);
                if let Some(attr) = var.attrs.into_iter().find(|a| a.path.is_ident("params")) {
                    let parsed_meta = attr.parse_meta().unwrap();
                    if let syn::Meta::List(list) = parsed_meta {
                        for item in list.nested {
                            if let syn::NestedMeta::Meta(meta) = item {
                                match meta {
                                    syn::Meta::Path(_) => {}
                                    syn::Meta::List(_) => {}
                                    syn::Meta::NameValue(value) => {
                                        if value.path.is_ident("path") {
                                            if let syn::Lit::Str(val) = value.lit {
                                                path = val.value();
                                            }
                                        } else if value.path.is_ident("view") {
                                            if let syn::Lit::Str(val) = value.lit {
                                                view = val.value();
                                            }
                                        } else if value.path.is_ident("auth") {
                                            if let syn::Lit::Bool(val) = value.lit {
                                                auth = val.value;
                                            }
                                        } else if value.path.is_ident("navbar") {
                                            if let syn::Lit::Bool(val) = value.lit {
                                                navbar = val.value;
                                            }
                                        } else if value.path.is_ident("sidebar") {
                                            if let syn::Lit::Bool(val) = value.lit {
                                                sidebar = val.value;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                let field_type = match var.fields {
                    syn::Fields::Unnamed(fields_unnamed) => {
                        let field_names =
                            fields_unnamed.unnamed.iter().enumerate().map(|(index, _)| {
                                Ident::new(&format!("__field_{}", index), Span::call_site())
                            });

                        let args = field_names.clone();
                        quote! {
                            #enum_ident::#ident(#(#field_names),*) => {
                                fn get_exact(a: MatcherToken) -> Option<String> {
                                    if let MatcherToken::Exact(s) = a {
                                        Some(s)
                                    } else {
                                        None
                                    }
                                }

                                let end_path = vec![#(#args.clone(),)*];
                                let mt = parse_str_and_optimize_tokens(#path, FieldNamingScheme::Unnamed).unwrap();
                                let exact = mt.into_iter().find_map(get_exact).unwrap_or(String::new());

                                let new_path = format!("{}{}", exact, end_path.join("/"));

                                Route {
                                    route: new_path,
                                    state: AppRouteState {
                                        auth: #auth,
                                        navbar: #navbar,
                                        sidebar: #sidebar
                                    }
                                }
                            }
                        }
                    }
                    syn::Fields::Unit => quote! {
                        #enum_ident::#ident => {
                            Route {
                                route: #path.to_string(),
                                state: AppRouteState {
                                    auth: #auth,
                                    navbar: #navbar,
                                    sidebar: #sidebar
                                }
                            }
                        }
                    },
                    syn::Fields::Named(_) => {
                        panic!("This derive is unsupported for using Named field")
                    }
                };

                variant_checker_functions.extend(field_type);

                routes.push(RouteMeta {
                    name: ident.to_string(),
                    path,
                    view,
                    auth,
                    navbar,
                    sidebar,
                })
            }
        }
        _ => panic!("#[derive(RouteDerive)] can only be used with enums"),
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #enum_ident #ty_generics #where_clause {
            pub fn get_route(&self) -> Route<AppRouteState> {
                match self {
                    #variant_checker_functions
                }
            }
        }

        impl Into<Route<AppRouteState>> for #enum_ident {
            fn into(self) -> Route<AppRouteState> {
                self.get_route()
            }
        }

        impl ToString for #enum_ident {
            fn to_string(&self) -> String {
                self.get_route().to_string()
            }
        }
    };

    let tts = {
        let mut arms = vec![];
        let mut matchers = vec![];
        let mut metas = vec![];
        for route_meta in routes.into_iter() {
            let path = route_meta.path;
            let auth = route_meta.auth;
            let navbar = route_meta.navbar;
            let sidebar = route_meta.sidebar;
            let view = Ident::new(&route_meta.view, Span::call_site());
            arms.push(quote! {
                #path => html!{ <views::#view/> },
            });
            matchers.push(quote! {
                routes.push(RouteMatcher::new(#path, setting).unwrap());
            });
            metas.push(quote! {
                meta.insert(#path.to_string(), AppRouteState {
                    auth: #auth,
                    navbar: #navbar,
                    sidebar: #sidebar,
                });
            });
        }
        let match_arms = TokenStream2::from_iter(arms.into_iter());
        let route_matchers = TokenStream2::from_iter(matchers.into_iter());
        let route_metas = TokenStream2::from_iter(metas.into_iter());

        quote! {
            use std::collections::HashMap;
            use yew_router_route_parser::{parse_str_and_optimize_tokens, FieldNamingScheme, MatcherToken};

            impl AppRouteHandler {
                pub fn new() -> Self {
                    let setting = MatcherSettings::default();
                    let except: RouteMatcher = RouteMatcher::new("/", setting).unwrap();
                    let mut routes: Vec<RouteMatcher> = vec![];
                    let mut meta: Box<HashMap<String, AppRouteState>> = Box::new(HashMap::new());

                    #route_matchers
                    routes = routes.into_iter().filter(|r| *r != except).collect();

                    #route_metas

                    AppRouteHandler {
                        routes,
                        meta,
                    }
                }

                pub fn get_routes(&self) -> Vec<RouteMatcher> {
                    self.routes.clone()
                }

                pub fn get_route_state(&self, input: &str) -> AppRouteState {
                    let path = match route_parser::parse(input, self.routes.clone()) {
                        Some((path, _)) => path,
                        None => input.to_string(),
                    };

                    self.meta.get(&path).map(|m| m.clone()).unwrap_or(AppRouteState::default())
                }

                pub fn render_view(&self, input: Route<AppRouteState>) -> Html {
                    let path = match route_parser::parse(&input, self.routes.clone()) {
                        Some((path, _)) => path,
                        None => input.to_string(),
                    };

                    match path.as_ref() {
                        #match_arms
                        _ => html! {
                            <views::NotFound />
                        }
                    }
                }
            }
        }
    };

    let mut token = TokenStream::from(tts);
    token.extend(TokenStream::from(expanded));

    token
}
