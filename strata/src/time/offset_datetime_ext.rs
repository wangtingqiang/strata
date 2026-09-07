use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

/// OffsetDateTime 扩展。
pub trait OffsetDateTimeExt {
    /// 转为 UTC 的 PrimitiveDateTime。
    fn to_primitive_utc(&self) -> PrimitiveDateTime;
    /// 毫秒时间戳。
    fn unix_timestamp_millis(&self) -> i64;
    /// 从毫秒时间戳构建。
    fn from_unix_timestamp_millis(
        millis: i64,
    ) -> Result<OffsetDateTime, time::error::ComponentRange>;
}

impl OffsetDateTimeExt for OffsetDateTime {
    fn to_primitive_utc(&self) -> PrimitiveDateTime {
        let value = self.to_offset(UtcOffset::UTC);
        PrimitiveDateTime::new(value.date(), value.time())
    }

    fn unix_timestamp_millis(&self) -> i64 {
        self.unix_timestamp() * 1_000 + i64::from(self.millisecond())
    }

    fn from_unix_timestamp_millis(
        millis: i64,
    ) -> Result<OffsetDateTime, time::error::ComponentRange> {
        OffsetDateTime::from_unix_timestamp_nanos(millis as i128 * 1_000_000)
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, offset, time};

    use super::*;

    #[test]
    fn from_unix_timestamp_millis_roundtrip() {
        let now = OffsetDateTime::now_utc();
        let millis = now.unix_timestamp_millis();
        let restored = OffsetDateTime::from_unix_timestamp_millis(millis).unwrap();
        assert_eq!(restored.unix_timestamp_millis(), millis);
    }

    #[test]
    fn unix_timestamp_millis_is_exact_including_fraction() {
        let value = OffsetDateTime::new_utc(date!(2024 - 06 - 01), time!(12:34:56.123));
        assert_eq!(value.unix_timestamp_millis(), 1_717_245_296_123);
    }

    #[test]
    fn to_primitive_utc_converts_offset_datetime() {
        // 2024-06-02 01:00:00 (+08:00) 对应的 UTC 瞬时是 2024-06-01 17:00:00。
        let value =
            OffsetDateTime::new_utc(date!(2024 - 06 - 01), time!(17:00)).to_offset(offset!(+8));

        assert_eq!(
            value.to_primitive_utc(),
            PrimitiveDateTime::new(date!(2024 - 06 - 01), time!(17:00))
        );
    }

    #[test]
    fn to_primitive_utc_keeps_utc_untouched() {
        let value = OffsetDateTime::new_utc(date!(2024 - 06 - 01), time!(12:34:56));

        assert_eq!(
            value.to_primitive_utc(),
            PrimitiveDateTime::new(date!(2024 - 06 - 01), time!(12:34:56))
        );
    }

    #[test]
    fn from_unix_timestamp_millis_rejects_out_of_range() {
        assert!(OffsetDateTime::from_unix_timestamp_millis(i64::MAX).is_err());
    }
}
