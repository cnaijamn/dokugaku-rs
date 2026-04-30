use rand::Rng;
use std::io::{Write};

const NMATERS: i32 = 3;

enum Mode {
    First,
    Second,
    Ok,
    Fail,
}

struct Game {
    maters: Vec<i32>,
    m: Mode,
    hope_number: i32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            maters: vec![0; NMATERS as usize],
            m: Mode::First,
            hope_number: -1,
        }
    }

    fn init_maters(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..self.maters.len() {
            self.maters[i] = rng.gen_range(5..25);
        }
    }

    fn draw_maters(&self) {
        println!("Maters\n");

        for i in 0..self.maters.len() {
            print!("{i}: ");
            for _ in 0..self.maters[i] {
                print!("|");
            }
            println!("");
        }
        println!("");
    }

    fn draw(&mut self) {
        // Unixの場合:
        let _ = std::process::Command::new("clear").status();
        // Winの場合:
        //TODO

        match self.m {
            Mode::First => {
                self.draw_maters();
                println!("どのメーターが一番早くなくなるか番号を指定してください。");
                print!("番号 > ");
                std::io::stdout().flush().expect("failed to flush");
            }
            Mode::Second => {
                self.draw_maters();
            }
            Mode::Ok => {
                self.draw_maters();
                println!("正解です！");
            }
            Mode::Fail => {
                self.draw_maters();
                println!("不正解です。。");
            }
        }
    }

    fn update(&mut self) -> bool {
        match self.m {
            Mode::First => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).expect("failed to read line");
                self.hope_number = buf.trim().parse::<i32>().expect("failed to parse buf");
                self.m = Mode::Second;
            }
            Mode::Second => {
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..self.maters.len());
                self.maters[index] -= 1;
                if self.maters[index] <= 0 {
                    if index as i32 == self.hope_number {
                        self.m = Mode::Ok;
                    } else {
                        self.m = Mode::Fail;
                    }
                }
            }
            Mode::Ok => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).expect("failed to read line");
                self.init_maters();
                self.m = Mode::First;
            }
            Mode::Fail => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).expect("failed to read line");
                self.init_maters();
                self.m = Mode::First;
            }
        }
        true
    }

    pub fn run(&mut self) {
        self.init_maters();

        loop {
            self.draw();
            if !self.update() {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(33));
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
