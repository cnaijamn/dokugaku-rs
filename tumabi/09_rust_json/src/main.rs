// https://www.youtube.com/watch?v=xv3LJgN3hY0&list=PL1tg_vSL2RVM5QRWLv4MkdOvUv1fDy0Zh&index=10

/*
# serdeをderive featureを有効にして導入
% cargo add serde --features derive

# JSONに変換するcreateであるserde_jsonを導入
% cargo add serde_json
*/

use serde_json;
use serde::{Serialize, Deserialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Animal {
    name: String,
    age: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct PTag {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DivTag {
    p_tags: Vec<PTag>,
}

fn main() {
    {
        let animal = Animal {
            name: String::from("Tama"),
            age: 20,
        };

        let serialized = serde_json::to_string(&animal).unwrap();
        println!("serialized: {}", serialized);

        let deserialized: Animal = serde_json::from_str(&serialized).unwrap();
        println!("deserialized: {:?}", deserialized);
    }
    println!("----------");
    {
        let mut div_tag = DivTag { p_tags: vec![] };
        div_tag.p_tags.push(PTag{ content: String::from("hige") });
        div_tag.p_tags.push(PTag{ content: String::from("hoge") });

        let serialized = serde_json::to_string(&div_tag).unwrap();
        println!("serialized: {}", serialized);

        let deserialized: DivTag = serde_json::from_str(&serialized).unwrap();
        println!("deserialized: {:?}", deserialized);
    }
    println!("----------");
    {
        let mut div_tag = DivTag { p_tags: vec![] };
        div_tag.p_tags.push(PTag{ content: String::from("fuga") });
        div_tag.p_tags.push(PTag{ content: String::from("fuge") });

        let serialized = serde_json::to_string(&div_tag).unwrap();
        println!("serialized: {}", serialized);

        let mut fout = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("data.json")
            .unwrap();
        fout.write_all(serialized.as_bytes()).unwrap();

        /*
        % cat ./data.json
        {"p_tags":[{"content":"fuga"},{"content":"fuge"}]}
        */

        let content = std::fs::read_to_string("data.json").unwrap();

        let deserialized: DivTag = serde_json::from_str(&content).unwrap();
        println!("deserialized: {:?}", deserialized);
    }
}
