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

        let mut result = Vec::new();
        for line in buffer.lines() {
            result.push(line.to_owned());
        }

        result
    }

    fn parse_input(raw_input: Vec<String>) -> Vec<i32> {
        let mut parsed_input = Vec::new();
        for line in raw_input{
            parsed_input.push(line.parse().unwrap());
        }

        parsed_input
    }
}

pub mod module_counting{
    pub fn count_fuel_needed(mass: &i32) -> f64 {
        substract_two(round_down(divide_by_three((*mass).into())))
    }

    fn divide_by_three(input: f64) -> f64 {
        input / 3.0
    }

    fn round_down(input: f64) -> f64 {
        use math::round;
        round::floor(input, 0)
    }

    fn substract_two(input: f64) -> f64 {
        input - 2.0
    }
}