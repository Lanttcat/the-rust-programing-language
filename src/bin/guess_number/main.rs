use std::io;
use rand::Rng;
use std::cmp::Ordering;

// 书中的代码有的老，需要自己查文档确定每个use的api。
fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..100);

    loop {
        let mut guess= String::new();
        std::println!("Please input you number:");

        // 什么时候考虑在设计一个函数参数的时候用&
        // 1 在函数被调用之后，被希望被释放
        // 2 函数不必获取所有权
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
