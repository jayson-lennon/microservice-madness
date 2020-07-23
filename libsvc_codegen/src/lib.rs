use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, FnArg, Ident, ItemFn, ItemStruct, Pat, PatType, PathArguments, ReturnType,
    Type, TypePath,
};

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
    let function = parse_macro_input!(input as ItemFn);

    let signature = function.clone().sig;

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

                match return_type.arguments {
                    PathArguments::AngleBracketed(ty) => {
                        let inner = ty.args.first().unwrap().clone();
                        quote! { #inner }
                    }
                    _ => panic!("Return value for action must be a Result<T>"),
                }
            }
            _ => panic!("Must have a return type"),
        }
    };

    let mut service_client_var_name = String::new();

    let mut fn_args = vec![];

    for arg in signature.inputs.iter() {
        println!("===========================");
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
        panic!("missing ServiceClient reference");
    }

    let server_function = {
        let mut function = function.clone();
        function.sig.ident = Ident::new("action_local", signature.ident.span());
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
            ArgType::Owned(ty) => {
                let fn_ident = format_ident!("{}", arg.0);
                let struct_ident = format_ident!("_{}", arg.0);
                Some(quote! { #struct_ident: #fn_ident })
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let client_fn_sig = function.clone().sig;

    let tokens = quote! {
        #server_function

        #client_fn_sig {
            #[derive(Serialize)]
            struct _Params {
                #(#struct_members),*
            }
            let endpoint = broker::get_endpoint(SERVICE_NAME, usvc_client).await?;
            let params = _Params { #(#fn_idents),* };

            let response = usvc_client.request(&params, &endpoint.address).await?;
            let response: #return_type = serde_json::from_str(&response)?;
            Ok(response)
        }
    };
    println!("token stream = {:#?}", tokens.to_string());

    TokenStream::from(tokens)
}

#[proc_macro_derive(Microservice)]
pub fn microservice(input: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(input as ItemStruct);
    let tokens = quote!( { #tokens });
    println!("{:?}", tokens.to_string());
    TokenStream::from(tokens)
}
