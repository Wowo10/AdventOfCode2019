extern crate day2;

#[cfg(test)]
mod test_module {

    use day2::functions;
    #[test]
    fn test1() {
        testcase("2,0,0,0,99", "1,0,0,0,99");
    }
    #[test]
    fn test2() {
        testcase("2,3,0,6,99", "2,3,0,3,99");
    }
    #[test]
    fn test3() {
        testcase("2,4,4,5,99,9801", "2,4,4,5,99,0");
    }
    #[test]
    fn test4() {
        testcase("30,1,1,4,2,5,6,0,99", "1,1,1,4,99,5,6,0,99");
    }

    fn testcase(expected: &str, input_str: &str) {
        let input = String::from(input_str);

        let mut intcode =
            functions::file_utils::parse_input(functions::file_utils::split_input(input));

        functions::intcodes::handle_intcode(&mut intcode);

        let mut joined = join(&intcode);
        joined.pop();

        assert_eq!(expected, joined);
    }

    fn join(a: &Vec<i32>) -> String {
        use std::fmt::Write;

        a.iter().fold(String::new(), |mut s, &n| {
            write!(s, "{},", n).ok();
            s
        })
    }
}
