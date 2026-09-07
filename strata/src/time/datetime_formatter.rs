use time::{
    OffsetDateTime, PrimitiveDateTime, format_description::well_known::Rfc3339, macros::offset,
};

const DEFAULT_RFC3339_UTC8: &str = "1970-01-01T08:00:00+08:00";

/// 日期时间格式化扩展（UTC+8）。
pub trait DateTimeFormatter {
    /// 格式化为人读日期时间（UTC+8）。
    fn to_human_friendly_utc8(&self) -> String;
    /// 格式化为人读日期时间（UTC+8，无秒）。
    fn to_human_friendly_no_seconds_utc8(&self) -> String;
    /// 格式化为 RFC3339（UTC+8）。
    fn to_rfc3339_utc8(&self) -> String;
}

impl DateTimeFormatter for OffsetDateTime {
    fn to_human_friendly_utc8(&self) -> String {
        format_human_friendly_datetime(self.to_offset(offset!(+8)))
    }

    fn to_human_friendly_no_seconds_utc8(&self) -> String {
        format_human_friendly_datetime_no_seconds(self.to_offset(offset!(+8)))
    }

    fn to_rfc3339_utc8(&self) -> String {
        format_rfc3339_utc8(self.to_offset(offset!(+8)))
    }
}

impl DateTimeFormatter for PrimitiveDateTime {
    fn to_human_friendly_utc8(&self) -> String {
        format_human_friendly_datetime(self.assume_utc().to_offset(offset!(+8)))
    }

    fn to_human_friendly_no_seconds_utc8(&self) -> String {
        format_human_friendly_datetime_no_seconds(self.assume_utc().to_offset(offset!(+8)))
    }

    fn to_rfc3339_utc8(&self) -> String {
        format_rfc3339_utc8(self.assume_utc().to_offset(offset!(+8)))
    }
}

fn format_human_friendly_datetime(value: OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        value.year(),
        u8::from(value.month()),
        value.day(),
        value.hour(),
        value.minute(),
        value.second()
    )
}

fn format_human_friendly_datetime_no_seconds(value: OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        value.year(),
        u8::from(value.month()),
        value.day(),
        value.hour(),
        value.minute(),
    )
}

fn format_rfc3339_utc8(value: OffsetDateTime) -> String {
    match value.format(&Rfc3339) {
        Ok(value) => value,
        Err(_) => DEFAULT_RFC3339_UTC8.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use time::{OffsetDateTime, macros::datetime};

    use crate::time::datetime_parser::DateTimeParser;

    use super::*;

    #[test]
    fn human_friendly_roundtrip() {
        let now = OffsetDateTime::now_utc();
        let formatted = now.to_human_friendly_utc8();
        let parsed = formatted.parse_human_friendly_utc8().unwrap();
        assert_eq!(parsed.to_human_friendly_utc8(), formatted);
    }

    #[test]
    fn rfc3339_has_utc8_offset() {
        let formatted = OffsetDateTime::now_utc().to_rfc3339_utc8();
        assert!(formatted.ends_with("+08:00"));
    }

    #[test]
    fn rfc3339_formats_fixed_timestamp() {
        let value = datetime!(2024-06-01 12:34:56 UTC);
        assert_eq!(value.to_rfc3339_utc8(), "2024-06-01T20:34:56+08:00");
    }

    #[test]
    fn human_friendly_formats_fixed_timestamp() {
        let value = datetime!(2024-06-01 12:34:56 UTC);
        assert_eq!(value.to_human_friendly_utc8(), "2024-06-01 20:34:56");
    }

    #[test]
    fn human_friendly_no_seconds_formats_fixed_timestamp() {
        let value = datetime!(2024-06-01 12:34:56 UTC);
        assert_eq!(
            value.to_human_friendly_no_seconds_utc8(),
            "2024-06-01 20:34"
        );
    }

    #[test]
    fn cross_day_shift_when_converting_to_utc8() {
        let value = datetime!(2024-06-01 18:00:00 UTC);
        assert_eq!(value.to_human_friendly_utc8(), "2024-06-02 02:00:00");
    }

    #[test]
    fn primitive_datetime_formats_as_utc8() {
        let value = datetime!(2024-06-01 12:34:56);

        assert_eq!(value.to_rfc3339_utc8(), "2024-06-01T20:34:56+08:00");
        assert_eq!(value.to_human_friendly_utc8(), "2024-06-01 20:34:56");
        assert_eq!(
            value.to_human_friendly_no_seconds_utc8(),
            "2024-06-01 20:34"
        );
    }
}
