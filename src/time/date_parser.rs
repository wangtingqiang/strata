use time::{Date, error::Parse, format_description::FormatItem, macros::format_description};

const ISO_DATE_FORMAT: &[FormatItem<'static>] = format_description!("[year]-[month]-[day]");

pub trait DateParser {
    fn parse_iso_date(&self) -> Result<Date, Parse>;
}

impl DateParser for str {
    fn parse_iso_date(&self) -> Result<Date, Parse> {
        Date::parse(self, ISO_DATE_FORMAT)
    }
}
