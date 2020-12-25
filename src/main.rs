use std::io;

fn main() {

  let secret_num = rand::random::<i32>();

  println!("{}", secret_num);

  println!("Guess the number!");

  println!("Input your guess");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  println!("You guessed: {}", guess);
}
