mod functions;
use functions::file_utils::*;

fn main() {
    let input = get_input();

    for i in input {
        println!("{}", i);
    }

    println!("TheEnd");
}
