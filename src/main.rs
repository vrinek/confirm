use std::io;
use std::process;

fn main() {
    println!("Are you sure? (yes/NO)");

    let answer = get_answer();

    if is_positive(answer) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn get_answer() -> String {
    let mut answer = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Failed to read line");

    answer.trim().to_lowercase()
}

fn is_positive(answer: String) -> bool {
    answer == "yes" || answer == "y"
}
