use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Guessing game");
    let secret_number=rand::thread_rng().gen_range(1..=100);
    println!("Enter ur guess");
    let mut guess=String::new();
   
    io::stdin()
     .read_line(&mut guess)
     .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please enter a number");
    println!("U guessed {guess}");
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too low"),
        Ordering::Greater => println!("Too high"),
        Ordering::Equal => println!("You won the game"),
    }
}
