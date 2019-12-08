mod functions;

fn main() {
    let input = functions::get_input();

    for line in &input{
        println!("{}", line);
    }
}


