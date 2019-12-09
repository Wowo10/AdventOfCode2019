mod functions;
use functions::file_utils::*;
use functions::intcodes::*;

fn main() {
    let input_dupa = get_input();
    //part1
    println!("Result: {}", count_result(&input_dupa, 12, 2));
    //part2
    const DESIREDVALUE: i32 = 19690720;

    for i in 0..=99 {
        for j in 0..=99 {
            let result = count_result(&input_dupa, i, j);
            if result == DESIREDVALUE {
                println!("Noun:{}, Verb:{}", i, j);
                println!("Result:{}", 100 * i + j);
                return;
            }
        }
    }
}

fn count_result(input: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut input = input.to_owned();

    change_value(&mut input, 1, noun);
    change_value(&mut input, 2, verb);

    handle_intcode(&mut input);

    get_value(&input, 0)
}
