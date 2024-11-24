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

    if let None = name {
        name = Some(fn_name.to_string());
    }

    let expanded = quote! {
        #visibility fn #fn_name(#inputs) #output {
            #block
        }

        /// Metadata for omu. **DO NOT CALL DIRECTLY**
        #visibility fn #meta_fn_name() -> String {
            #name.to_string()
        }
    };

    TokenStream::from(expanded)
}
