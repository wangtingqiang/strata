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

#[cfg(test)]
mod tests {
    use time::{UtcOffset, macros::datetime};

    use super::*;

    #[test]
    fn parses_human_friendly_with_utc8_offset() {
        let parsed = "2024-06-01 12:34:56".parse_human_friendly_utc8().unwrap();

        assert_eq!(
            parsed.to_offset(UtcOffset::UTC),
            datetime!(2024-06-01 04:34:56 UTC)
        );
        assert_eq!(parsed.offset(), offset!(+8));
    }

    #[test]
    fn parses_human_friendly_no_seconds_with_utc8_offset() {
        let parsed = "2024-06-01 12:34"
            .parse_human_friendly_no_seconds_utc8()
            .unwrap();

        assert_eq!(
            parsed.to_offset(UtcOffset::UTC),
            datetime!(2024-06-01 04:34:00 UTC)
        );
        assert_eq!(parsed.offset(), offset!(+8));
    }

    #[test]
    fn rejects_malformed_datetime() {
        assert!("2024-6-1 12:34:56".parse_human_friendly_utc8().is_err());
        assert!(
            "2024-06-01 12:34:56"
                .parse_human_friendly_no_seconds_utc8()
                .is_err()
        );
        assert!("not a date".parse_human_friendly_utc8().is_err());
        assert!("not a date".parse_human_friendly_no_seconds_utc8().is_err());
    }
}
