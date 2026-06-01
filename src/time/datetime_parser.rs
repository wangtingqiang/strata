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

pub trait DateTimeParser {
    fn parse_human_friendly_utc8(&self) -> Result<OffsetDateTime, Parse>;
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
