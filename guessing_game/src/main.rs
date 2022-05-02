use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("guess the number!");

    println!("the secret number is {}", secret_number);

    loop {
        println!("please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //next iteration of outer loop
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!");
                break; // break outer loop
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}