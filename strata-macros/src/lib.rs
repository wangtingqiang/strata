use proc_macro::TokenStream;

#[proc_macro_derive(ToErrorInfo, attributes(info))]
pub fn derive_to_error_info(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
