use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType, ReturnType, Type, TypePath};

#[derive(Debug, Clone)]
enum ArgType {
    Owned(proc_macro2::TokenStream),
    Ref(String),
}

fn get_arg_type_path(ty: &Type) -> TypePath {
    match ty.clone() {
        Type::Path(p) => p,
        _ => panic!("unable to find arg type"),
    }
}

fn get_arg_type_name(ty: &Type) -> String {
    match ty.clone() {
        Type::Path(p) => p.path.segments.first().unwrap().ident.to_string(),
        _ => panic!("unable to find arg type name"),
    }
}

fn get_arg_type(ty: &Type) -> ArgType {
    match ty.clone() {
        Type::Path(_) => {
            let mut tokens = proc_macro2::TokenStream::new();
            get_arg_type_path(ty).to_tokens(&mut tokens);
            ArgType::Owned(tokens)
        }
        Type::Reference(r) => ArgType::Ref(get_arg_type_name(&r.elem)),
        _ => panic!("unhandle argument type"),
    }
}

fn get_arg_name(t: &PatType) -> String {
    match *t.pat.clone() {
        Pat::Ident(i) => i.ident.to_string(),
        _ => panic!("unable to get param name"),
    }
}

#[proc_macro_attribute]
pub fn remote(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_str = attr
        .clone()
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("");

    let (src_path, module_path) = {
        let components: Vec<_> = attr_str.split(";").collect();
        (components[0], components[1])
    };

    let (crate_name, module_path) = {
        let components: Vec<_> = module_path.split("::").collect();
        (components[0], components[1])
    };

    let crate_name = format_ident!("{}", crate_name);
    let module_path = format_ident!("{}", module_path);

    let src_path = {
        let mut path = PathBuf::from(src_path);
        path.set_extension("rs");
        path
    };

    let function = parse_macro_input!(input as ItemFn);

    let signature = function.clone().sig;

    let fn_name = signature.clone().ident.to_string();

    let target_path = PathBuf::from(&format!(
        "fizzbuzz/src/bin/svc-{}-{}.rs",
        src_path.as_path().file_stem().unwrap().to_str().unwrap(),
        fn_name
    ));

    let return_type = {
        match signature.output {
            ReturnType::Type(_, ty) => {
                let return_type = get_arg_type_path(&ty)
                    .path
                    .segments
                    .first()
                    .unwrap()
                    .clone();
                if return_type.ident != "Result" {
                    panic!("Action function must return a Result<T>");
                }
                quote! { #return_type }
            }
            _ => panic!("Must have a return type"),
        }
    };

    let mut service_client_var_name = String::new();

    let mut fn_args = vec![];

    for arg in signature.inputs.iter() {
        match arg {
            FnArg::Typed(t) => {
                let arg_name = get_arg_name(t);
                let arg_type = get_arg_type(&t.ty);
                match arg_type {
                    ArgType::Owned(_) => fn_args.push((arg_name, arg_type)),
                    ArgType::Ref(ref t) => {
                        if t != "ServiceClient" {
                            panic!("Only ServiceClient may be used as a reference");
                        } else {
                            service_client_var_name = arg_name;
                        }
                    }
                }
            }
            _ => panic!(),
        }
    }

    if service_client_var_name == "" {
        panic!("Missing ServiceClient reference");
    }

    let service_client_var_name = format_ident!("{}", service_client_var_name);

    let server_function = {
        let mut function = function.clone();
        function.sig.ident = format_ident!("{}_impl", fn_name);
        function
    };

    let struct_members = fn_args
        .iter()
        .filter_map(|arg| match arg.1.clone() {
            ArgType::Owned(ty) => {
                let ident = format_ident!("_{}", arg.0);
                Some(quote! { #ident: #ty })
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let fn_idents = fn_args
        .iter()
        .filter_map(|arg| match arg.1.clone() {
            ArgType::Owned(_) => {
                let fn_ident = format_ident!("{}", arg.0);
                let struct_ident = format_ident!("_{}", arg.0);
                Some(quote! { #struct_ident: #fn_ident })
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let client_fn_sig = function.clone().sig;

    let target_struct = quote! {
        #[derive(Serialize, Deserialize, Debug)]
        struct _Params {
            #(#struct_members),*
        }
    };

    let client_tokens = quote! {
        #server_function

        pub #client_fn_sig {
            #target_struct
            let endpoint = libsvc::broker::get_endpoint(#fn_name, #service_client_var_name).await?;
            let params = _Params { #(#fn_idents),* };

            let response = #service_client_var_name.request(&params, &endpoint.address).await?;
            let response: #return_type = serde_json::from_str(&response)?;
            response
        }
    };

    // Write service file
    {
        let server_fn_name = server_function.clone().sig.ident;
        let fn_name_as_str = fn_name.to_string();
        let fn_args = fn_args
            .iter()
            .filter_map(|arg| match arg.1.clone() {
                ArgType::Owned(_) => {
                    let struct_ident = format_ident!("_{}", arg.0);
                    Some(quote! { params.#struct_ident })
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let service_src = quote! {
            #[macro_use]
            extern crate log;
            use dotenv::dotenv;
            use libsvc::{broker, ServiceClient};
            use rand::Rng;
            use std::net::SocketAddr;
            use serde::{Deserialize, Serialize};
            use std::convert::Infallible;
            use warp::Filter;

            use #crate_name::#module_path::#server_fn_name;

            #[derive(Clone)]
            pub struct State {
                client: ServiceClient,
            }

            #target_struct

            fn json_body() -> impl Filter<Extract = (_Params,), Error = warp::Rejection> + Clone {
                warp::body::content_length_limit(1024 * 16).and(warp::body::json())
            }

            async fn process_request(params: _Params, state: State) -> Result<impl warp::Reply, Infallible> {
                let client = state.client.clone();
                let result = #server_fn_name(#(#fn_args),* , &client).await;
                Ok(warp::reply::json(&result))
            }

            fn with_state(
                state: State,
            ) -> impl Filter<Extract = (State,), Error = std::convert::Infallible> + Clone {
                warp::any().map(move || state.clone())
            }

            #[tokio::main]
            async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
                dotenv().ok();

                env_logger::init();

                let mut rng = rand::thread_rng();

                let svc_client = ServiceClient::default();
                let state = State {
                    client: svc_client,
                };
                let svc_client = state.client.clone();
                let host = "127.0.0.1";

                let route = warp::any()
                    .and(warp::post())
                    .and(json_body())
                    .and(with_state(state))
                    .and_then(process_request);

                loop {
                    let port: u16 = rng.gen_range(30000, 50000);

                    let bind_addr: SocketAddr = match format!("{}:{}", host, port).parse() {
                        Ok(addr) => addr,
                        Err(_) => continue,
                    };

                    let bound_server = warp::serve(route.clone()).try_bind_ephemeral(bind_addr);

                    match bound_server {
                        Ok(server) => {
                            trace!("Bind successful @ {}. Running server...", &bind_addr);
                            broker::add_endpoint(#fn_name_as_str, &format!("http://{}/", bind_addr), &svc_client).await?;
                            server.1.await;
                            break;
                        }
                        Err(e) => {
                            debug!("Failed to bind to {} - {}. Retrying.", &bind_addr, e);
                            continue;
                        }
                    }
                }
                Ok(())
            }
        };

        let service_src_str = service_src.to_string();
        let mut target_file =
            File::create(&target_path).expect(&format!("failed to create file: {:?}", target_path));
        target_file
            .write_all(service_src_str.as_bytes())
            .expect("failed to write content to file");
    }

    TokenStream::from(client_tokens)
}
