use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..100);

    loop {
        let mut guess= String::new();
        std::println!("Please input you number:");
        io::stdin().read_line(&mut guess).expect("Invalid Input, Please again!");
        let guess_number: u32 = guess.trim().parse().expect("Invalid Input, Please again!");

        match guess_number.cmp(&random_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("Yes!!");
                break;
            }
        }
    }
}
