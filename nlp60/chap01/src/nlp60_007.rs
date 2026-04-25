use std::io;

fn main() {
    println!("摂氏(C)を入力してください:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");

    // 文字列を浮動小数点に変換
    let c: f64 = input.trim().parse().expect("数字を入力してください");
    let f = c * 1.8 + 32.0;

    println!("{} 度(C) は {} 度(K) です", c, f);
}
