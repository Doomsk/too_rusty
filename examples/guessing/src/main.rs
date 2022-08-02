use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);

    loop {
        println!("Type a number between 1 and 100");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("I wasn't possible to read the line.");

        let guess: i32 = match input.trim()     // remove the `\n` at the end of the string
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Go higher"),
            Ordering::Greater => println!("Go lower"),
            Ordering::Equal => {
                println!("That's it!");
                break;
            },
        }
    }
}
