use rand::Rng;
use std::process::Command;
use std::thread;
use std::time::Duration;

const CANVAS_WIDTH: i32 = 60;
const CANVAS_HEIGHT: i32 = 30;
const NRAINS: i32 = 10; // 雨粒の数

struct Rain {
    x: i32,
    y: i32,
    len: i32,
    amount_y: i32,
}

impl Rain {
    pub fn new() -> Self {
        Rain {
            x: 0,
            y: 0,
            len: 0,
            amount_y: 0,
        }
    }
}

struct World {
    canvas: Vec<char>,
    rains: Vec<Rain>,
}

impl World {
    pub fn new() -> Self {
        World {
            canvas: vec![],
            rains: vec![],
        }
    }

    fn init(&mut self) {
        for _ in 0..CANVAS_HEIGHT {
            for _ in 0..CANVAS_WIDTH {
                self.canvas.push(' ');
            }
        }

        let mut rng = rand::thread_rng();

        for _ in 0..NRAINS {
            let mut rain = Rain::new();
            rain.x = rng.gen_range(0..CANVAS_WIDTH);
            rain.y = rng.gen_range(0..CANVAS_HEIGHT);
            rain.len = rng.gen_range(1..5);
            rain.amount_y = rng.gen_range(1..3);
            self.rains.push(rain);
        }
    }

    // 雨は下に移動
    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // 雨つぶをクリア
        for i in 0..self.canvas.len() {
            self.canvas[i] = ' ';
        }

        for rain in self.rains.iter_mut() {
            rain.y += rain.amount_y;

            if rain.y >= CANVAS_HEIGHT {
                // 表示外に出るなら、雨粒を移動
                rain.x = rng.gen_range(0..CANVAS_WIDTH);
                rain.y = 0;
            }

            for i in 0..rain.len {
                let index = ((rain.y + i) * CANVAS_WIDTH + rain.x) as usize;
                if index >= self.canvas.len() {
                    continue;
                }
                self.canvas[index] = '|'; // 雨つぶ
            }
        }

    }

    fn draw(&mut self) {
        // 端末表示クリア
        // Unixの場合:
        let status = Command::new("clear").status();
        // Winの場合:
        //let status = Command::new("cmd").args(&["/C", "cls"]).status();

        if let Err(err) = status {
            eprintln!("failed to clear: {}", err);
            std::process::exit(1);
        }

        for y in 0..CANVAS_HEIGHT {
            for x in 0..CANVAS_WIDTH {
                let i = (y * CANVAS_WIDTH + x) as usize;
                let c = self.canvas[i];
                print!("{}", c);
            }
            println!("");
        }
    }

    pub fn run(&mut self) {
        self.init();

        loop {
            self.update();
            self.draw();
            thread::sleep(Duration::from_millis(33));
        }
    }
}

fn main() {
    let mut world = World::new();
    world.run();
}
