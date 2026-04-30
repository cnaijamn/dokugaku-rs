struct Body {
    text: String,
}

impl Body {
    pub fn new(text: &str) -> Self {
        Body {
            text: String::from(text),
        }
    }
}

fn pad(max_len: usize, text: &String) -> String {
    let n = (max_len - text.len()) / 2;
    let mut add = 0;
    if text.len() % 2 == 1 {
        add = 1;
    }
    return format!("{}{}{}", " ".repeat(n), text, " ".repeat(n + add));
}

fn draw_daruma(daruma: &Vec<Body>) {
    let max_len = 20;
    let s = format!("+{}+", "-".repeat(max_len));
    println!("{}", s);
    for body in daruma.iter() {
        let text = pad(max_len, &body.text);
        println!("|{}|", text);
        println!("{}", s);
    }
}

fn main() {
    let mut daruma: Vec<Body> = vec![];

    daruma.push(Body::new("(^o^)"));
    daruma.push(Body::new("(>v<)"));
    daruma.push(Body::new("(-w-)"));

    loop {
        draw_daruma(&daruma);

        print!("Number > ");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("failed to read line");
        println!("");

        let index = buf.trim().parse::<usize>().unwrap();
        //println!("### index = {}", index);
        if index >= daruma.len() {
            eprintln!("index out of range");
            continue;
        }
        daruma.remove(index);

        if daruma.len() <= 0 {
            println!("End");
            break;
        }
    }
}
