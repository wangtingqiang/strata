use time::OffsetDateTime;

use crate::{http::api::ResponseBody, time::DateTimeFormatter};

pub(in crate::http::api) fn success_body(message: impl Into<String>) -> ResponseBody<()> {
    ResponseBody {
        success: true,
        code: "0".to_owned(),
        message: message.into(),
        time: Some(OffsetDateTime::now_utc().to_rfc3339_utc8()),
        data: None,
    }
}

pub(in crate::http::api) fn failure_body(
    code: impl Into<String>,
    message: impl Into<String>,
) -> ResponseBody<()> {
    ResponseBody {
        success: false,
        code: code.into(),
        message: message.into(),
        time: Some(OffsetDateTime::now_utc().to_rfc3339_utc8()),
        data: None,
    }
}
