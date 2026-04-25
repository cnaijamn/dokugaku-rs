use rand::Rng;

fn main() {
    let fortunes = ["大吉", "忠吉", "凶"];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..fortunes.len());

    println!("今日の運勢は... {} です", fortunes[index]);
}
