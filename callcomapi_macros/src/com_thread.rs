use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, ItemFn, parse_macro_input};

// Lightweight proc-macro: generate a wrapper that delegates execution to
// `callcomapi_runtime`. The macro ensures parameter/return types are
// `Send + 'static` (compile-time checks) and maps the attribute (STA/MTA)
// to the runtime `ComModel`.

pub fn inner_com_thread(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse and normalize attribute (accepts STA/MTA variants)
    let model_kind_str = if attr.is_empty() || attr.to_string().trim().is_empty() {
        "STA"
    } else {
        let ident: syn::Ident = match syn::parse(attr.clone()) {
            Ok(i) => i,
            Err(_) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "invalid attribute syntax; expected STA or MTA without quotes",
                )
                .to_compile_error()
                .into();
            }
        };
        match ident.to_string().to_uppercase().as_str() {
            "MTA" | "MULTI" | "MULTITHREADED" => "MTA",
            "STA" | "APARTMENT" | "APARTMENTTHREADED" => "STA",
            _ => {
                return syn::Error::new_spanned(ident, "invalid COM model, expected STA or MTA")
                    .to_compile_error()
                    .into();
            }
        }
    };

    // parse the original function and extract signature pieces
    let func = parse_macro_input!(item as ItemFn);
    let vis = &func.vis;
    let sig = &func.sig;
    let block = &func.block;

    let inputs = &sig.inputs;
    let output = &sig.output;

    // collect types for compile-time Send + 'static assertions
    let arg_types: Vec<_> = inputs
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Typed(pat) => Some(&*pat.ty),
            _ => None,
        })
        .collect();

    let is_async = sig.asyncness.is_some();

    // generate compile-time assertions enforcing `Send + 'static`
    let mut assert_bounds = Vec::new();
    for (idx, arg_type) in arg_types.iter().enumerate() {
        let assert_fn_name = Ident::new(
            &format!("_assert_param_{}_is_send_static", idx),
            Span::call_site(),
        );
        assert_bounds.push(quote! {
            const fn #assert_fn_name() {
                const fn require<T: Send + 'static>() {}
                const fn check() { require::<#arg_type>(); }
            }
            let _ = #assert_fn_name;
        });
    }

    let ret_type_for_assert = match output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };
    let assert_ret_fn = Ident::new("_assert_return_is_send_static", Span::call_site());
    assert_bounds.push(quote! {
        const fn #assert_ret_fn() {
            const fn require<T: Send + 'static>() {}
            const fn check() { require::<#ret_type_for_assert>(); }
        }
        let _ = #assert_ret_fn;
    });

    let compile_time_checks = if assert_bounds.is_empty() {
        quote! {}
    } else {
        quote! { #(#assert_bounds)* }
    };

    // map normalized attribute to runtime enum
    let runtime_model_token = if model_kind_str == "MTA" {
        quote! { callcomapi_runtime::ComModel::MTA }
    } else {
        quote! { callcomapi_runtime::ComModel::STA }
    };

    // generate wrapper that delegates to runtime; parameters are captured
    // by `move` into the task closure so ownership moves across threads.
    let expanded = if is_async {
        quote! {
            #vis #sig {
                #compile_time_checks
                callcomapi_runtime::call_async(#runtime_model_token, move || {
                    futures::executor::block_on(async move { #block })
                }).await
            }
        }
    } else {
        quote! {
            #vis #sig {
                #compile_time_checks
                callcomapi_runtime::call_sync(#runtime_model_token, move || { (|| #block)() })
            }
        }
    };

    expanded.into()
}
