extern crate day3;

#[cfg(test)]
mod test_module {

    use day3::functions;
    #[test]
    fn test1() {
        functions::file_utils::parse_file(String::from("R8,U5,L5,D3\nU7,R6,D4,L4"));

        //assert_eq!();
    }

    //TODO: Add parsing to string and check for equality
}
