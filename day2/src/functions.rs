pub mod file_utils {
    pub fn get_input() -> Vec<i32> {
        parse_input(load_file_by_lines())
    }

    fn load_file_by_lines() -> Vec<String> {
        use std::fs;
        use std::path::Path;

        let pathstr = String::from("data/input");
        let path = Path::new(&pathstr);

        let buffer = fs::read_to_string(path).expect("Failed to initialise file read.");

        let buffer: Vec<&str> = buffer.split(',').collect();

        buffer.iter().map(|a: &&str| String::from(*a)).collect()
    }

    fn parse_input(raw_input: Vec<String>) -> Vec<i32> {
        let mut parsed_input = Vec::new();
        for line in raw_input {
            parsed_input.push(line.parse().unwrap());
        }

        parsed_input
    }
}
