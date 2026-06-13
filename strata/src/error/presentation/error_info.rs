use std::fmt::{Debug, Display};

use crate::error::presentation::ErrorKind;

pub trait ErrorInfo: Debug + Display {
    fn kind(&self) -> ErrorKind;
    fn code(&self) -> &'static str;
    fn message(&self) -> &'static str;

    fn is_severe(&self) -> bool {
        self.kind().is_severe()
    }
}
