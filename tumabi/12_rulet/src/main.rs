use std::io::Write;

enum Mode {
    First,
    Second,
    Third,
}

struct Game {
    m: Mode,
    items: Vec<String>,
    cursor: i32,
    seed: f32,
    loop_max: i32,
    time: i32,
}

impl Game {
    pub fn new() -> Self {
        // 初期化
        Game {
            m: Mode::First,
            items: vec![
                String::from("当たり"),
                String::from("ハズレ"),
                String::from("罰ゲーム"),
            ],
            cursor: 0,
            seed: 0.0,
            loop_max: 0,
            time: 0,
        }
    }

    fn sleep(&self) {
        let mils = std::time::Duration::from_millis(330);
        std::thread::sleep(mils);
    }

    pub fn input(&mut self) {
        let mut line = String::new();
        print!("input > ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        self.seed = line.trim().parse::<f32>().unwrap();
        self.loop_max = (self.seed * 3.14) as i32; // 適当な乱数シード
    }

    pub fn update(&mut self) -> bool {
        match self.m {
            Mode::First => {
                self.input();
                self.m = Mode::Second;
                true
            }
            Mode::Second => {
                self.sleep();
                if self.time >= self.loop_max {
                    self.m = Mode::Third;
                }
                self.cursor = (self.cursor + 1) % self.items.len() as i32;
                self.time += 1;
                true
            }
            Mode::Third => {
                false
            }
        }
    }

    fn clear(&self) {
        // ターミナル表示をクリア
        std::process::Command::new("clear").status().unwrap();
    }

    pub fn draw(&self) {
        self.clear();

        for i in 0..self.items.len() {
            let item = &self.items[i];
            if i == self.cursor as usize {
                println!("-> {}", item)
            } else {
                println!("   {}", item)
            }
        }
    }
}

fn main() {
    let mut game = Game::new();

    while game.update() {
        game.draw();
    }
}
