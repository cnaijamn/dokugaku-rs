use std::io::{ stdin, stdout, Write };
use std::str::{ FromStr };

struct Animal {
    name: String,
    age: i32,
}

impl Animal {
    pub fn new(name: &str, age: i32) -> Self {
        Animal {
            name: name.to_string(),
            age: age,
        }
    }

    pub fn show(&self) {
        println!("{} {}", self.name, self.age);
    }
}

fn show_animals(animals: &Vec<Animal>) {
    for animal in animals.iter() {
        animal.show();
    }
}

fn main() {
    let mut animals: Vec<Animal> = vec![];

    loop {
        let mut buf = String::new();
        print!("input > ");
        stdout().flush().unwrap();

        let bytes = stdin().read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }

        //println!("{}", buf);
        let toks: Vec<&str> = buf.trim_end().split(",").collect();
        //println!("{:?}", toks);
        let name: &str = toks[0];
        let age: i32 = FromStr::from_str(toks[1]).unwrap();
        //println!("{} {}", name, age);

        let animal: Animal = Animal::new(name, age);
        animals.push(animal);

        show_animals(&animals);
    };
}
