use std::io;

fn main() {
    println!("Welcome to guess number!");
    println!("input the number you guess:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect(
        "Failed to read line"
    );
    println!("The number you guess is:{}",guess);
}
