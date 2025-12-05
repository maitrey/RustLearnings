use rand::prelude::*;

fn main() {
    let rand_num = rand::rng().random_range(1..=100);
    println!("{:#?}", rand_num);

    use text_io::read;
    let mut guess_num: i32;

    loop{
        let input_str : String = read!();
        match input_str.trim().parse::<i32>() {
            Ok(n) =>  {
                guess_num = n;
                println!("Guessed number is: {:#?}", guess_num);
                if guess_num < rand_num {
                    println!("Too low");
                } else if guess_num > rand_num {
                    println!("Too high!");
                } else {
                    println!("Congratulations You guessed it right");
                    break;
                }
            },
            Err(_) => {
                println!("Its a non-integer!");
                continue
            }
        }
    }

}
