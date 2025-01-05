use rand::Rng;
use std::{
    io,
    env,
    cmp::Ordering,
};

fn parse_args() -> (i32, i32) {
    let args: Vec<String> = env::args().collect();
    let mut min = 0;
    let mut max = 100;
    if let Some(val_max) = args.get(2) {
        max = val_max.trim().parse().expect("Wrong max value");
        min = args.get(1)
            .expect("ERROR: unknown")
            .trim().parse()
            .expect("Wrong max value");
    } else if let Some(val_max) = args.get(1) {
        max = val_max.trim().parse().expect("Wrong max value");
    }
    (min, max)
}

fn main() {
    let (min, max) = parse_args();
    println!("Welcome to number guess.\nmin: {min},\nmax: {max}");
    loop {
        println!("Starting a new game, number is ??");
        let num = rand::thread_rng().gen_range(min..=max);
        loop {
            let mut input = String::new();
            print!("Enter guess: ");
            match io::stdin().read_line(&mut input) {
                Err(_) => println!("ERRPR: Could not read input."),
                Ok(_) => match input.trim().parse::<i32>() {
                    Err(_) => println!("Wrong input"),
                    Ok(guess) => match guess.cmp(&num) {
                        Ordering::Less => println!("Number too low"),
                        Ordering::Greater => println!("Number too high"),
                        Ordering::Equal => {
                            println!("Gotcha! you won");
                            break;
                        }
                    }
                }
            }
        }
    }
}
