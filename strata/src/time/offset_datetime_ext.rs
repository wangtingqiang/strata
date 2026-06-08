use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

pub trait OffsetDateTimeExt {
    fn to_primitive_utc(&self) -> PrimitiveDateTime;
    fn unix_timestamp_millis(&self) -> i64;
}

impl OffsetDateTimeExt for OffsetDateTime {
    fn to_primitive_utc(&self) -> PrimitiveDateTime {
        let value = self.to_offset(UtcOffset::UTC);
        PrimitiveDateTime::new(value.date(), value.time())
    }

    fn unix_timestamp_millis(&self) -> i64 {
        self.unix_timestamp() * 1_000 + i64::from(self.millisecond())
    }
}
