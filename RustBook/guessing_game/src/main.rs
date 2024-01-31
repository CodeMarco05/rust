use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generate a number between 1 and 100

    loop { // loop is a keyword for an infinity loop
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read inptu"); // inputs from cmd to the given adress for a varaible

        let guess: u8 = guess.trim().parse().expect("Please type a number"); //makes the guess varaible as a u32 the expect takes error handeling
        // initing a variable again is called shadowing (.trim()) is eliminating \n or \t when you press enter 
        // let guess :u32 = match guess.trim().parse(){
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        println!("Your guess {guess}");
        match guess.cmp(&secret_number) { //cmp takes comparing from the lib std::cmp::Ordering
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
        }
    }

    println!("The correct number was {secret_number}"); //you can make a placeholder for a variable in your output like this
}
