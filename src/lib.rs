pub mod network;

#[cfg(test)]
mod tests {
    use super::*;
    use network;

    #[test]
    fn test_network_ip() {
        let res = network::ip();
        println!("res: {:?}", res);
        assert!(res.is_some());
    }
}
