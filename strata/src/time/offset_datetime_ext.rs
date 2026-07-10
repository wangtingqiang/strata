use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

pub trait OffsetDateTimeExt {
    fn to_primitive_utc(&self) -> PrimitiveDateTime;
    fn unix_timestamp_millis(&self) -> i64;
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
    use super::*;

    #[test]
    fn from_unix_timestamp_millis_roundtrip() {
        let now = OffsetDateTime::now_utc();
        let millis = now.unix_timestamp_millis();
        let restored = OffsetDateTime::from_unix_timestamp_millis(millis).unwrap();
        assert_eq!(restored.unix_timestamp_millis(), millis);
    }
}
