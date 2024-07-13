#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
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

    // let main_func_name = format_ident!("__{}_main_func", arg.appname.as_ref().unwrap());
    let main_func_name = format_ident!("main");
    let mod_name = format_ident!("__app_init_{}_", arg.appname.as_ref().unwrap());
    let call_func_name = f.sig.ident.clone();

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
    }

    let attrs = f.attrs.clone();

    let origin = quote!(
        #(#attrs)*
        #f
    );

    let core = quote!(
        #[no_mangle]
        pub unsafe extern "C" fn #main_func_name(argc: u32, argv: *const *const u8) {
            use core::iter::Iterator;
            use rtsmart_std::param::ParamItem;
            let vec = {
                (0..argc as isize)
                    .map(|i| {
                        let mut len = 0usize;
                        loop {
                            if *(*argv.offset(i)).offset(len as isize) != b'\0' {
                                len += 1;
                            } else {
                                break
                            }
                        }
                        ParamItem::new(core::slice::from_raw_parts::<'static, _>(*argv.offset(i), len))
                    })
                    .collect::<Vec<_>>()
            };
            #call_func_name (vec.into_iter())
        }
    );

    quote!(
        #origin
        mod #mod_name {
            use super::#call_func_name;
            use core::marker::Sync;
            extern crate alloc;
            use alloc::vec::Vec;
            use core::iter::IntoIterator;

            #core
        }
    )
        .into()
}
