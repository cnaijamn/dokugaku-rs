//use rand::Rng;
use rand::RngExt;

fn main() {
    let fortunes = ["大吉", "中吉", "凶"];
    //let mut rng = rand::thread_rng();
    //let index = rng.gen_range(0..fortunes.len());
    let mut rng = rand::rng();
    let index = rng.random_range(0..fortunes.len());

    println!("今日の運勢は... {} です", fortunes[index]);
}
