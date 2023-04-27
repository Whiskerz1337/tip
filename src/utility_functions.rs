use colored::ColoredString;

pub fn user_confirmation(question: ColoredString) -> bool {
    println!("{}", question);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_lowercase() == "y"
}

pub fn parse_input_string(input: &str) -> String {
    let mut processed = String::new();

    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            processed.push(c.to_ascii_lowercase());
        }
    }

    processed
}
