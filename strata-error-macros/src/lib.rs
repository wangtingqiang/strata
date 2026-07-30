mod error_info;

#[proc_macro_derive(ErrorInfo, attributes(info))]
pub fn derive_error_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    error_info::derive_error_info_impl(input)
}
