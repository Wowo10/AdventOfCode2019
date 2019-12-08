mod functions;
use functions::file_utils::*;
use functions::module_counting::*;

fn main() {
    let input = get_input();

    for mass in &input{
        println!("{}", count_fuel_needed(mass));
    }
}


