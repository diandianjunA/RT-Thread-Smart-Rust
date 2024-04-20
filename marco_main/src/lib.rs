#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::ToTokens;
use syn::{parse, spanned::Spanned, ReturnType, Visibility};

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default)]
    appname: Option<String>,
    #[darling(default)]
    desc: Option<String>,
}

#[proc_macro_attribute]
pub fn marco_main_use(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let raw_arg = parse_macro_input!(args as syn::AttributeArgs);
    let parg = Args::from_list(&raw_arg).map_err(|e| e.write_errors());
    let arg = match parg {
        Ok(x) => x,
        Err(e) => {
            return e.into();
        }
    };

    if arg.appname.is_none() {
        return parse::Error::new(
            Span::call_site(),
            "`#[marco_main_use]` macro must have attribute `appname`",
        )
            .to_compile_error()
            .into();
    }

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.unsafety.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.len() == 1
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
        ReturnType::Default => true,
        _ => false,
    };
    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `fn(arg: vec::IntoIter<&[u8]>)`",
        )
            .to_compile_error()
            .into();
    };
    let content = f.block.into_token_stream();
    let core = quote!(
        #[no_mangle]
        pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> usize {
            #content
            0
        }
    );

    quote!(
        #core
    ).into()
}
