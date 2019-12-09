pub mod file_utils {
    #[derive(Debug)]
    pub enum Direction {
        Up(i32),
        Right(i32),
        Down(i32),
        Left(i32),
    }

    pub fn get_input() -> [Vec<Direction>; 2] {
        parse_file(load_file())
    }

    pub fn parse_file(file_content: String) -> [Vec<Direction>; 2] {
        let lines = split_by_lines(file_content);

        let lines_separated: Vec<Vec<String>> = lines.iter().map(|x| split_by_commas(x)).collect();

        let first_row: Vec<Direction> = lines_separated[0]
            .iter()
            .map(|x| parse_direction(x))
            .collect();

        let second_row: Vec<Direction> = lines_separated[1]
            .iter()
            .map(|x| parse_direction(x))
            .collect();

        [first_row, second_row]
    }

    fn split_by_commas(raw_input: &String) -> Vec<String> {
        let buffer: Vec<&str> = raw_input.split(',').collect();

        buffer.iter().map(|a: &&str| String::from(*a)).collect()
    }

    fn parse_direction(text: &String) -> Direction {
        let temp: String = text.chars().skip(1).collect();
        match text.chars().next().unwrap() {
            'U' | 'u' => {
                return Direction::Up(parse_direction_len(temp));
            }
            'R' | 'r' => {
                return Direction::Right(parse_direction_len(temp));
            }
            'D' | 'd' => {
                return Direction::Down(parse_direction_len(temp));
            }
            'L' | 'l' => {
                return Direction::Left(parse_direction_len(temp));
            }
            _ => panic!("Wrong direction string."),
        }
    }

    fn parse_direction_len(text: String) -> i32 {
        text.parse().unwrap()
    }

    fn load_file() -> String {
        use std::fs;
        use std::path::Path;

        let pathstr = String::from("data/input");
        let path = Path::new(&pathstr);

        fs::read_to_string(path).expect("Failed to initialise file read.")
    }

    fn split_by_lines(buffer: String) -> Vec<String> {
        let mut result = Vec::new();
        for line in buffer.lines() {
            result.push(line.to_owned());
        }

        result
    }
}
