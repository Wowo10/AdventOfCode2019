pub fn get_input() -> Vec<String> {
    load_file_by_lines()
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