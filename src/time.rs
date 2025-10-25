pub fn now_timestamp_nanos() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should not go backwards")
        .as_nanos()
}

pub fn year_month_day() -> String {
    jiff::Zoned::now().strftime("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_year_month_day() {
        let ymd = super::year_month_day();
        println!("year_month_day: {}", ymd);
    }
}
