//use rand::Rng;
use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    //let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = rand::rng().random_range(1..=100);

    //println!("{}", secret_number);
    println!("1から100の数字を当ててね！");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("入力エラー");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("もっと大きいよ"),
            Ordering::Greater => println!("もっと小さいよ"),
            Ordering::Equal => {
                println!("正解！");
                break;
            }
        }
    }
}
