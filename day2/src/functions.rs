pub mod file_utils {
    pub fn get_input() -> Vec<i32> {
        parse_input(split_input(load_file()))
    }

    pub fn split_input(raw_input: String) -> Vec<String> {
        let buffer: Vec<&str> = raw_input.split(',').collect();

        buffer.iter().map(|a: &&str| String::from(*a)).collect()
    }

    pub fn parse_input(raw_input: Vec<String>) -> Vec<i32> {
        let mut parsed_input = Vec::new();
        for line in raw_input {
            parsed_input.push(line.parse().unwrap());
        }

        parsed_input
    }

    fn load_file() -> String {
        use std::fs;
        use std::path::Path;

        let pathstr = String::from("data/input");
        let path = Path::new(&pathstr);

        fs::read_to_string(path).expect("Failed to initialise file read.")
    }
}

pub mod intcodes {
    pub fn get_value(intcode: &Vec<i32>, position: usize) -> i32 {
        intcode[position]
    }

    pub fn change_value(intcode: &mut Vec<i32>, position: usize, value: i32) {
        intcode[position] = value;
    }

    pub fn handle_intcode(intcode: &mut Vec<i32>) {
        let mut counter = 0;

        while !handle_next(intcode, counter) {
            counter += 4;
        }
    }

    fn handle_next(intcode: &mut Vec<i32>, current_position: usize) -> bool {
        match intcode[current_position] {
            1 => {
                handle_chunk(intcode, current_position, &add_function);
            }
            2 => {
                handle_chunk(intcode, current_position, &multiply_function);
            }
            99 => {
                return true;
            }
            _ => panic!("Wrong opcode!"),
        }

        false
    }

    fn add_function(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
    fn multiply_function(num1: i32, num2: i32) -> i32 {
        num1 * num2
    }

    fn handle_chunk(
        intcode: &mut Vec<i32>,
        current_position: usize,
        num_closure: &dyn Fn(i32, i32) -> i32,
    ) {
        let value1 = get_value(&intcode, intcode[current_position + 1] as usize);
        let value2 = get_value(&intcode, intcode[current_position + 2] as usize);
        let position_to_replace = intcode[current_position + 3] as usize;
        change_value(intcode, position_to_replace, num_closure(value1, value2));
    }
}
