pub mod get;

#[cfg(test)]
mod tests {
    use super::*;
    use get;

    #[test]
    fn test_get_to_string() {
        let res = get::to_string();
        println!("res: {:?}", res);
        assert!(res.is_some());
    }
}
