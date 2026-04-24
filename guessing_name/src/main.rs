use std::io;

fn main(){
    println!("GUESS NUMBER!");

    println!("Please input your GUESS!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}