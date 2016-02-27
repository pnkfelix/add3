extern crate add6;
extern crate add6_64;

#[cfg(test)]
mod tests {
    use add6;
    use add6_64;

    #[test]
    fn it_works() {
        assert_eq!(add6::add6(10), 16_i32);
        assert_eq!(add6_64::add6(10), 16_i64);
    }
}
