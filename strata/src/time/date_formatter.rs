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
