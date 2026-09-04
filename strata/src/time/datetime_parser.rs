use time::{
    OffsetDateTime, PrimitiveDateTime,
    error::Parse,
    format_description::FormatItem,
    macros::{format_description, offset},
};

const HUMAN_FRIENDLY_DATETIME_FORMAT: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

const HUMAN_FRIENDLY_DATETIME_NO_SECONDS_FORMAT: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]");

/// 日期时间解析扩展。
pub trait DateTimeParser {
    /// 解析 `yyyy-MM-dd HH:mm:ss`，按 UTC+8 解释。
    fn parse_human_friendly_utc8(&self) -> Result<OffsetDateTime, Parse>;
    /// 解析 `yyyy-MM-dd HH:mm`，按 UTC+8 解释。
    fn parse_human_friendly_no_seconds_utc8(&self) -> Result<OffsetDateTime, Parse>;
}

impl DateTimeParser for str {
    fn parse_human_friendly_utc8(&self) -> Result<OffsetDateTime, Parse> {
        PrimitiveDateTime::parse(self, HUMAN_FRIENDLY_DATETIME_FORMAT)
            .map(|value| value.assume_offset(offset!(+8)))
    }

    fn parse_human_friendly_no_seconds_utc8(&self) -> Result<OffsetDateTime, Parse> {
        PrimitiveDateTime::parse(self, HUMAN_FRIENDLY_DATETIME_NO_SECONDS_FORMAT)
            .map(|value| value.assume_offset(offset!(+8)))
    }
}
