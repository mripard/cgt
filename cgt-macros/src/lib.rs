use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro]
pub fn cgt_assert(item: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = item.into();

    quote! {
        if !(#input) {
            return Err(TestError::ConditionUnmet(stringify!(#input).to_string()));
        }
    }
    .into()
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn cgt_test(args: TokenStream, item: TokenStream) -> TokenStream {
    if !args.is_empty() {
        let args: proc_macro2::TokenStream = args.into();

        emit_error! { args, "This macro doesn't use any attribute" };
    }

    let input = parse_macro_input!(item as ItemFn);
    let fn_ident = &input.sig.ident;
    let fn_name = fn_ident.to_string();

    quote! {
        #input

        inventory::submit!(
            cgt_core::Test {
                module_name: module_path!(),
                test_name: #fn_name,
                test_fn: cgt_core::TestFunction::NoArg(#fn_ident),
            }
        );
    }
    .into()
}

#[proc_macro_attribute]
pub fn cgt_test_with_fd(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_ident = &input.sig.ident;
    let fn_name = fn_ident.to_string();

    quote! {
        #input

        inventory::submit!(
            cgt_core::Test {
                module_name: module_path!(),
                test_name: #fn_name,
                test_fn: TestFunction::WithFd(#fn_ident),
            }
        );
    }
    .into()
}
