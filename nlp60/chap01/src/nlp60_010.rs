use std::fs;
use std::io;

fn main() {
    println!("メモしたい内容を入力してください:");

    let mut memo = String::new();
    io::stdin().read_line(&mut memo).expect("入力エラー");

    let filename = "memo.txt";
    fs::write(filename, memo).expect("書き込みエラー");

    println!("'{}' に保存しました！", filename);
}
