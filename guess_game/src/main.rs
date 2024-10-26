use std::io; // importing std::io module

fn main() {
    println!("Guess the game is here!");
    println!("You can guess a number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read the line!");
    
    println!("You have guessed {guess}");
}
