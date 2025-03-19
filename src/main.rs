use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess...");

    let mut guess = String::new();
    let secret_number= rand::rng().random_range(1..=100);
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess:u32 =guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!.Secret number is = {}",secret_number),
        Ordering::Greater => println!("Too big!Secret number is = {}",secret_number),
        Ordering::Equal => println!("You win!Secret number is = {}",secret_number),
    }
}

