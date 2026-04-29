use std::collections::HashMap;

fn main() {
    let mut book = HashMap::new();
    book.insert("PCサポート", "0120-000-000");
    book.insert("緊急", "110");

    match book.get("緊急") {
        Some(number) => println!("電話番号: {}", number),
        None => println!("登録されていません"),
    }
}
