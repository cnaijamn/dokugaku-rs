use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    // Rust -> JSON
    let user = User {
        name: String::from("Taro"),
        age: 30,
    };
    let json = serde_json::to_string(&user).unwrap();
    println!("JSON: {}", json);

    // JSON -> Rust
    let data = r#"{"name":"Hanako","age":25}"#;
    let user2: User = serde_json::from_str(data).unwrap();
    println!("復元: {:?}", user2);
}
