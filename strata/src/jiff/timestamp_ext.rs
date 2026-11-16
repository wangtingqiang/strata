use jiff::{
    Timestamp,
    tz::{Offset, TimeZone},
};

const FORMAT_RFC3339: &str = "%Y-%m-%dT%H:%M:%S%:z";
const FORMAT_HUMAN_FRIENDLY: &str = "%Y-%m-%d %H:%M:%S";
const FORMAT_HUMAN_FRIENDLY_NO_SECONDS: &str = "%Y-%m-%d %H:%M";

/// UTC+8 时区。
fn utc8() -> TimeZone {
    TimeZone::fixed(Offset::constant(8))
}

/// `Timestamp` 的 UTC+8 格式化扩展。
pub trait TimestampExt {
    /// 格式化为 RFC3339（UTC+8）。
    fn to_rfc3339_utc8(&self) -> String;

    /// 格式化为人读日期时间（UTC+8）。
    fn to_human_friendly_utc8(&self) -> String;

    /// 格式化为人读日期时间（UTC+8，无秒）。
    fn to_human_friendly_no_seconds_utc8(&self) -> String;
}

impl TimestampExt for Timestamp {
    fn to_rfc3339_utc8(&self) -> String {
        self.to_zoned_datetime(utc8())
            .strftime(FORMAT_RFC3339)
            .to_string()
    }

    fn to_human_friendly_utc8(&self) -> String {
        self.to_zoned_datetime(utc8())
            .strftime(FORMAT_HUMAN_FRIENDLY)
            .to_string()
    }

    fn to_human_friendly_no_seconds_utc8(&self) -> String {
        self.to_zoned_datetime(utc8())
            .strftime(FORMAT_HUMAN_FRIENDLY_NO_SECONDS)
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timestamp(value: &str) -> Timestamp {
        value.parse().unwrap()
    }

    #[test]
    fn formats_rfc3339_with_utc8_offset() {
        assert_eq!(
            timestamp("2024-06-01T12:34:56Z").to_rfc3339_utc8(),
            "2024-06-01T20:34:56+08:00"
        );
    }

    #[test]
    fn formats_human_friendly() {
        let value = timestamp("2024-06-01T12:34:56Z");

        assert_eq!(value.to_human_friendly_utc8(), "2024-06-01 20:34:56");
        assert_eq!(
            value.to_human_friendly_no_seconds_utc8(),
            "2024-06-01 20:34"
        );
    }

    #[test]
    fn shifts_to_the_next_day() {
        assert_eq!(
            timestamp("2024-06-01T18:00:00Z").to_human_friendly_utc8(),
            "2024-06-02 02:00:00"
        );
    }
}
