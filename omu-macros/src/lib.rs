use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn slash(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut name: Option<String> = None;

    let meta_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("name") {
            name = Some(meta.value()?.parse::<LitStr>()?.value());
            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    });
    parse_macro_input!(args with meta_parser);

    let r#fn = parse_macro_input!(input as ItemFn);

    let fn_name = &r#fn.sig.ident;
    let meta_fn_name = Ident::new(&format!("__omu_{}_metadata", fn_name), Span::call_site());

    let block = &r#fn.block;
    let visibility = &r#fn.vis;
    let inputs = &r#fn.sig.inputs;
    let output = &r#fn.sig.output;
    let asynchronous = &r#fn.sig.asyncness;

    if asynchronous.is_none() {
        return syn::Error::new_spanned(quote!(#[slash]), "handler must be async")
            .to_compile_error()
            .into();
    }

    if name.is_none() {
        name = Some(fn_name.to_string());
    }

    let expanded = quote! {
        #visibility async fn #fn_name(#inputs) #output {
            #block
        }

        /// Metadata for omu. **DO NOT CALL DIRECTLY**
        #visibility fn #meta_fn_name() -> String {
            #name.to_string()
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn event(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut name: Option<String> = None;

    let meta_parser = syn::meta::parser(|meta| {
        if name.is_none() {
            name = Some(meta.path.get_ident().unwrap().to_string());

            match name.clone().unwrap().as_str() {
                "ready" => Ok(()),
                _ => Err(meta.error(format!("unsupported event name {}", name.clone().unwrap()))),
            }
        } else {
            Err(meta.error(format!(
                "event name already set to {}",
                name.to_owned().unwrap()
            )))
        }
    });
    parse_macro_input!(args with meta_parser);

    if name.is_none() {
        return syn::Error::new_spanned(quote!(#[event]), "event name not set")
            .to_compile_error()
            .into();
    }

    let r#fn = parse_macro_input!(input as ItemFn);

    let fn_name = &r#fn.sig.ident;
    let meta_fn_name = Ident::new(&format!("__omu_{}_metadata", fn_name), Span::call_site());

    let block = &r#fn.block;
    let visibility = &r#fn.vis;
    let inputs = &r#fn.sig.inputs;
    let output = &r#fn.sig.output;
    let asynchronous = &r#fn.sig.asyncness;
    // let types = inputs
    //     .into_iter()
    //     .filter_map(|arg| match arg {
    //         FnArg::Receiver(_) => None,
    //         FnArg::Typed(syn::PatType { ty, .. }) => Some(*ty.clone()),
    //     })
    //     .collect::<Vec<_>>();

    // for item in types {
    //     match item {
    //         syn::Type::Path(syn::TypePath { path, .. }) => match path.get_ident() {
    //             Some(ident) => match ident.to_string().as_str() {
    //                 "Context" => continue,
    //                 _ => (),
    //             },
    //             None => return syn::Error::new_spanned(quote!(#[event]), "unknown type").to_compile_error().into(),
    //         },
    //         _ => (),
    //     }
    // }

    if asynchronous.is_none() {
        return syn::Error::new_spanned(quote!(#[event]), "handler must be async")
            .to_compile_error()
            .into();
    }

    let expanded = quote! {
        use ijson::{ijson, IValue};

        #visibility async fn #fn_name(#inputs) #output {
            #block
        }

        /// Metadata for omu. **DO NOT CALL DIRECTLY**
        #visibility fn #meta_fn_name() -> IValue {
            let name = #name.to_string();
            ijson!({
                "name": name
            })
        }
    };

    TokenStream::from(expanded)
}
