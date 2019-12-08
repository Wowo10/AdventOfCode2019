extern crate day1;

#[cfg(test)]
mod test_module {

    use day1::functions;
    
    #[test]
    fn test1() {
        assert_eq!(4,functions::add_two(2));
    }
    
    #[test]
    fn test2() {
        assert_ne!(4,functions::add_two(3));
    }
}