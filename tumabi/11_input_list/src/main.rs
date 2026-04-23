//use std::{collections::LinkedList, io::Write};
use std::collections::LinkedList;
use std::io::Write;

fn main() {
    let mut list = LinkedList::<String>::new();

    loop {
        let mut line = String::new();

        print!("input > ");
        std::io::stdout().flush().unwrap();

        match std::io::stdin().read_line(&mut line) {
            Ok(size) => {
                if size <= 0 { //バグ？:エンターのみでもlineに改行が残る(sizeがゼロにはならない)
                    break;
                }
            }
            Err(err) => {
                eprint!("{}", err);
                std::process::exit(1);
            }
        }

        let args: Vec<&str> = line.trim().split(" ").collect();
        if args[0] == "show" {
            for item in list.iter() {
                print!("{} ", item);
            }
            println!("");
        } else if args[0] == "push_back" {
            list.push_back(args[1].to_string());
        } else if args[0] == "pop_back" {
            let item = list.pop_back();
            if !item.is_none() {
                println!("{}", item.unwrap());
            }
        } else if args[0] == "pop_front" {
            let item = list.pop_front();
            if !item.is_none() {
                println!("{}", item.unwrap());
            }
        }
    }
}
