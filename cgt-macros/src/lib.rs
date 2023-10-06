use attribute_derive::FromAttr;
use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, Expr, Ident, ItemFn, Token};

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

#[proc_macro]
pub fn cgt_assert_err(item: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = item.into();

    quote! {
        if !(#input).is_err() {
            return Err(TestError::ResultNotOk(format!("{:#?}", (#input))));
        }
    }
    .into()
}

#[proc_macro]
pub fn cgt_assert_ok(item: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = item.into();

    quote! {
        if !(#input).is_ok() {
            return Err(TestError::ResultNotOk(format!("{:#?}", (#input))));
        }
    }
    .into()
}

#[allow(dead_code)]
struct AssertionInput {
    left: Expr,
    comma: Token![,],
    right: Expr,
}

impl Parse for AssertionInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            left: input.parse()?,
            comma: input.parse()?,
            right: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn cgt_assert_eq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssertionInput);
    let left = &input.left;
    let right = &input.right;

    quote! {
        if (#left) != (#right) {
            return Err(TestError::NotEqual(format!("{:#?}", #left), format!("{:#?}", #right)));
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
                master: false,
                client_capabilities: [None; 8],
            }
        );
    }
    .into()
}

#[derive(Debug, FromAttr)]
struct TestAttributes {
    master: bool,

    #[attribute(optional)]
    capabilities: Vec<Ident>,
}

#[derive(Clone, Copy, Debug)]
enum ExplicitOption<T> {
    Some(T),
    None,
}

impl<T> From<Option<T>> for ExplicitOption<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => ExplicitOption::Some(v),
            None => ExplicitOption::None,
        }
    }
}

impl<T: ToTokens> ToTokens for ExplicitOption<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let ExplicitOption::Some(ref t) = *self {
            tokens.extend(quote! { Some(#t) })
        } else {
            tokens.extend(quote! { None })
        }
    }
}

#[proc_macro_attribute]
pub fn cgt_test_with_fd(args: TokenStream, item: TokenStream) -> TokenStream {
    let attrs: TestAttributes = parse_macro_input!(args);
    let master = attrs.master;

    if attrs.capabilities.len() > 8 {
        todo!();
    }

    let mut caps: [ExplicitOption<&Ident>; 8] = [ExplicitOption::None; 8];
    for (idx, caps) in caps.iter_mut().enumerate() {
        if idx >= 8 {
            break;
        }

        *caps = attrs.capabilities.get(idx).into();
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
                test_fn: cgt_core::TestFunction::WithFd(#fn_ident),
                master: #master,
                client_capabilities: [#(#caps),*]
            }
        );
    }
    .into()
}
