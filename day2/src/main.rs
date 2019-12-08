mod functions;
use functions::file_utils::*;
use functions::intcodes::*;

fn main() {
    let mut input = get_input();

    change_value(&mut input, 1, 12);
    change_value(&mut input, 2, 2);

    handle_intcode(&mut input);

    println!("Result: {}", get_value(&input, 0));
}
