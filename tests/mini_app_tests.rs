#![cfg(feature = "mini-app")]

#[cfg(test)]
mod tests {
    use telegram_rs_2::mini_app::InitData;

    #[test]
    fn test_init_data_timestamp_validation() {
        let raw = "query_id=123&auth_date=1700000000&hash=mockhash";
        let init_data = InitData::parse(raw).expect("parse failed");
        // An old timestamp should not be valid for a 60-second window
        assert!(!init_data.is_valid_timestamp(60));
    }
}
