use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Pat, PatType, Type};

#[derive(Debug)]
enum ArgType {
    Owned(String),
    Ref(String),
}

fn get_arg_type_name(ty: &Type) -> String {
    match ty.clone() {
        Type::Path(p) => p.path.segments.first().unwrap().ident.to_string(),
        _ => panic!("unable to find arg type name"),
    }
}

fn get_arg_type(ty: &Type) -> ArgType {
    match ty.clone() {
        Type::Path(_) => ArgType::Owned(get_arg_type_name(ty)),
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
pub fn microservice(attr: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let function_copy = function.clone();
    let sig = function.sig;
    let mut args = vec![];
    for arg in sig.inputs.iter() {
        println!("===========================");
        match arg {
            FnArg::Typed(t) => {
                let arg_name = get_arg_name(t);
                let arg_type = get_arg_type(&t.ty);
                match arg_type {
                    ArgType::Owned(_) => args.push((arg_name, arg_type)),
                    ArgType::Ref(ref t) => {
                        if t != "ServiceClient" {
                            panic!("Only ServiceClient may be used as a reference");
                        }
                    }
                }
            }
            _ => panic!(),
        }
    }

    println!("{:#?}", args);

    let server_function = {
        let mut function = function_copy.clone();
        function.sig.ident = Ident::new("action_server", sig.ident.span());
        function
    };

    let client_function = {
        // make struct here to send to server
    };

    let tokens = quote! {
        #server_function
    };
    println!("token stream = {:?}", tokens.to_string());

    TokenStream::from(tokens)
}
