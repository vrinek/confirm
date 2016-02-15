use std::io;
use std::process;
use std::env;

fn main() {
    let prompt = get_prompt();
    println!("{} (yes/NO)", prompt);

    let response = get_response();

    if is_positive(response) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

// Returns a custom provided prompt or the default one.
fn get_prompt() -> String {
    match env::args().nth(1) {
        Some(prompt) => prompt,
        None => String::from("Are you sure?")
    }
}

// Gets the response from STDIN and trims it.
fn get_response() -> String {
    let mut response = String::new();

    io::stdin().read_line(&mut response)
        .expect("Failed to read line");

    String::from(response.trim())
}

// Returns true for a positive response.
fn is_positive(response: String) -> bool {
    let response = response.to_lowercase();

    response == "yes" || response == "y"
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
