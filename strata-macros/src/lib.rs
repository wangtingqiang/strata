mod error;

use error::derive_to_error_info_impl;

#[proc_macro_derive(ToErrorInfo, attributes(info))]
pub fn derive_to_error_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_to_error_info_impl(input)
}
