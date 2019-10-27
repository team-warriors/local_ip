pub mod get;

#[cfg(test)]
mod tests {
    use super::*;
    use get;

    #[test]
    fn test_get_to_string() {
        assert_eq!(get::to_string(), Some(String::from("{\"local\":\"127.0.0.1\",\"network\":\"192.168.43.102\"}")));
    }
}
