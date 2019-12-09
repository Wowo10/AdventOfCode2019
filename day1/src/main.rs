mod functions;
use functions::file_utils::*;
use functions::module_counting::*;

fn main() {
    let input = get_input();

    let sum: f64 = input
        .iter()
        .map(|mass: &i32| count_fuel_needed(*mass))
        .sum();

    println!("Sum: {}", sum);
    let sum: f64 = input
        .iter()
        .map(|mass: &i32| count_total_fuel_needed(*mass))
        .sum();

    println!("Total sum: {}", sum);
}
