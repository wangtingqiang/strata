use time::Date;

/// 日期格式化扩展。
pub trait DateFormatter {
    /// 格式化为 ISO 日期（`yyyy-MM-dd`）。
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

#[cfg(test)]
mod tests {
    use time::macros::date;

    use crate::time::date_parser::DateParser;

    use super::*;

    #[test]
    fn iso_date_roundtrip() {
        let day = date!(2026 - 09 - 05);
        assert_eq!(day.to_iso_date().parse_iso_date().unwrap(), day);
    }
}
