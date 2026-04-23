// 例:
// % cargo run src/main.rs String
// 5: // % cargo run src/main.rs String
// 12:     let args: Vec<String> = env::args().collect();
// 20:     let fname: &String = &args[1];
// 21:     let keyword: &String = &args[2];

use std::fs::File;
use std::io::{ self, BufRead };
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    if args.len() < 3 {
        println!("Usage: cheapgrep [file-name] [keyword]");
        return;
    }
    //println!("{} {}", args[0], args[1]);

    let fname: &String = &args[1];
    let keyword: &String = &args[2];
    //println!("{} {}", fname, keyword);

    let file = File::open(fname).expect("file not found");
    let lines = io::BufReader::new(file).lines();
    let mut nlines = 1;

    for line in lines {
        let l = line.unwrap();
        //println!("{}", l);
        if l.contains(keyword) {
            println!("{}: {}", nlines, l);
        }
        nlines += 1;
    }
}
