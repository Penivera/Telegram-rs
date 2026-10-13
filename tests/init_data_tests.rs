#![cfg(feature = "mini-app")]

#[cfg(test)]
mod tests {
    use telegram_rs_2::mini_app::{InitData, UserData};

    #[test]
    fn test_init_data_parsing() {
        // Example data
        let raw = "query_id=AAG...&user=%7B%22id%22%3A123%2C%22first_name%22%3A%22John%22%2C%22is_bot%22%3Afalse%7D&auth_date=1234567890&hash=abc123hash";
        let data = InitData::parse(raw).expect("Parsing failed");

        assert_eq!(data.query_id.as_deref(), Some("AAG..."));
        assert_eq!(data.auth_date, 1234567890);
        assert_eq!(data.hash, "abc123hash");

        let user: UserData = data.user.expect("User data missing");
        assert_eq!(user.id, 123);
        assert_eq!(user.first_name, "John");
        assert_eq!(user.is_bot, Some(false));
    }
}
