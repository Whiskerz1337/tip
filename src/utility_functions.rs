use colored::ColoredString;

pub fn user_confirmation(question: ColoredString) -> bool {
    println!("{}", question);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_lowercase() == "y"
}
