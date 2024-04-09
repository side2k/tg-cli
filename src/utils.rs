use std::io;

pub fn request_input(prompt: &str) -> Option<String> {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    Some(String::from(input.trim()))
}
