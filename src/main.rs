use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("*** Guess the number! ***");

    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess
            .trim()
            .parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_num)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
}
