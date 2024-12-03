use rand::Rng; //trait
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜数!!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("猜一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("猜测的是：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("TOO SMALL"),
            Ordering::Greater => println!("to big"),
            Ordering::Equal => {
                println!("WIN");
                break;
            }
        }
    }
}
