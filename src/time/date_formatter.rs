use time::Date;

pub trait DateFormatter {
    fn to_iso_date(&self) -> String;
}

impl DateFormatter for Date {
    fn to_iso_date(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            self.year(),
            u8::from(self.month()),
            self.day()
        )
    }
}
