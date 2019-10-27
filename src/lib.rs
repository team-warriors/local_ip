pub mod get;

#[cfg(test)]
mod tests {
    use super::*;
    use get;

    #[test]
    fn test_get_to_string() {
        let expected = Some(String::from("{\"local\":\"127.0.0.1\",\"network\":\"192.168.43.102\"}"));
        assert_eq!(get::to_string(), expected);
    }
}
