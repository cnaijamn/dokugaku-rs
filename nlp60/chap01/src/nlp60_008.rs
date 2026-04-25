fn main() {
    let text = "Rust";

    // chars()で文字に分割後、rev()で反転、collect()で文字列に戻す
    let reversed: String = text.chars().rev().collect();

    println!("原文: {}", text);
    println!("反転: {}", reversed);
}
