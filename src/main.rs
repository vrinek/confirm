use std::io;
use std::process;
use std::env;

fn main() {
    let prompt = get_prompt();
    println!("{} (yes/NO)", prompt);

    let answer = get_answer();

    if is_positive(answer) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn get_prompt() -> String {
    match env::args().nth(1) {
        Some(prompt) => prompt,
        None => String::from("Are you sure?")
    }
}

// Gets the answer from STDIN and trims it.
fn get_answer() -> String {
    let mut answer = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Failed to read line");

    String::from(answer.trim())
}

// Returns true for a positive answer.
fn is_positive(answer: String) -> bool {
    let answer = answer.to_lowercase();

    answer == "yes" || answer == "y"
}

#[test]
fn returns_true_for_positive() {
    assert_eq!(is_positive(String::from("yes")), true);
    assert_eq!(is_positive(String::from("y")), true);
    assert_eq!(is_positive(String::from("YES")), true);
    assert_eq!(is_positive(String::from("Y")), true);
}

#[test]
fn returns_false_for_negative() {
    assert_eq!(is_positive(String::from("no")), false);
    assert_eq!(is_positive(String::from("n")), false);
    assert_eq!(is_positive(String::from("NO")), false);
    assert_eq!(is_positive(String::from("N")), false);
}

#[test]
fn returns_false_by_default() {
    assert_eq!(is_positive(String::from("")), false);
    assert_eq!(is_positive(String::from("I'm not sure")), false);
    assert_eq!(is_positive(String::from("maybe?")), false);
    assert_eq!(is_positive(String::from("👍")), false);
}
