// https://www.youtube.com/watch?v=56bAlKwSHWc

use std::io::{ stdin, stdout, Write };

struct StateMachine {
    move_mode: i32,
    is_jumping: bool,
    is_talking: bool,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            move_mode: 0,
            is_jumping: false,
            is_talking: false,
        }
    }

    pub fn parse(&mut self, input: &str) {
        //println!("parse[{}]", input);
        match input {
            "stop" => self.move_mode = 0,
            "walk" => self.move_mode = 10,
            "run" => self.move_mode = 20,
            "jump" => self.is_jumping = true,
            "talk" => self.is_talking = true,
            &_ => panic!("invalid input move_mode"),
        }
    }

    //pub fn show(&mut self) {
    pub fn show(&mut self) {
        match self.move_mode {
            0 => {
                println!("stopping...");
                self.is_talking = false;
            }
            10 => println!("I walking"),
            20 => println!("I running"),
            _ => println!("invalid move_mode"),
        }
        if self.is_jumping {
            println!("jump!");
            self.is_jumping = false;
        }
        if self.is_talking {
            println!("I talking")
        }
    }
}

fn main() {
    let mut buf = String::new();
    let mut sm = StateMachine::new();

    loop {
        print!("input > ");
        stdout().flush().unwrap();
        buf.clear();

        let bytes = stdin().read_line(&mut buf).unwrap();
        //println!("buf={}", buf);
        if bytes == 0 {
            break;
        }

        //println!("{}", buf);
        sm.parse(&buf.trim_end());
        sm.show();
    };
}
