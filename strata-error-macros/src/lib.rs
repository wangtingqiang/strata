//! `#[derive(ErrorInfo)]` 过程宏实现，供 strata-error 使用。

#![warn(missing_docs)]

mod error_info;

/// 为错误枚举派生 [`ErrorInfo`] 实现的过程宏。
///
/// 用法说明与示例见 strata-error crate 中 `ErrorInfo` 的 re-export 文档。
#[proc_macro_derive(ErrorInfo, attributes(info))]
pub fn derive_error_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    error_info::derive_error_info_impl(input)
}
