use jiff::Timestamp;
use strata::jiff::TimestampExt;

use crate::response::ResponseBody;

pub(in crate::response) fn success_body(message: impl Into<String>) -> ResponseBody<()> {
    ResponseBody {
        success: true,
        code: "0".to_owned(),
        message: message.into(),
        server_time: Timestamp::now().to_rfc3339_utc8(),
        data: None,
    }
}

pub(in crate::response) fn failure_body(
    code: impl Into<String>,
    message: impl Into<String>,
) -> ResponseBody<()> {
    ResponseBody {
        success: false,
        code: code.into(),
        message: message.into(),
        server_time: Timestamp::now().to_rfc3339_utc8(),
        data: None,
    }
}
