mod str_ext;
mod trim;

pub use str_ext::StrExt;
pub use trim::{TextError, trim_non_empty_bounded, trim_optional, trim_optional_bounded};
