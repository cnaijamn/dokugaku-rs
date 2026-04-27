use std::io::Write;

struct Factory {
    draw_buf: String,
    nboxes: u32,
}

impl Factory {
    pub fn new() -> Self {
        Factory {
            draw_buf: String::new(),
            nboxes: 0,
        }
    }

    pub fn run(&mut self) {
        while self.update() {
            self.draw();
        }

        self.draw();
    }

    pub fn draw(&self) {
        print!("{}", self.draw_buf);
    }

    pub fn update(&mut self) -> bool {
        print!("箱を作る(0) 退勤する(1) > ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(size) => {
                if size <= 0 {
                    return false;
                }
            }
            Err(err) => {
                eprintln!("{}", err);
                return false;
            }
        }

        let cmd = line.trim();
        match cmd {
            "0" => {
                self.add_box();
            }
            "1" => {
                self.taiking();
                return false;
            }
            //&_ => {
            _ => {
                println!("invalid command {}", cmd);
            }
        }

        true
    }

    pub fn add_box(&mut self) {
        self.nboxes += 1;

        self.draw_buf.clear();
        for i in 0..self.nboxes {
            self.draw_buf.push_str("■ ");
            if (i + 1) % 10 == 0 {
                self.draw_buf.push_str("\n");
            }
        }
        self.draw_buf.push_str("\n");
    }

    pub fn taiking(&mut self) {
        self.draw_buf = format!("工場長「お疲れ様。報酬はパン{}個だよ」\n", self.nboxes);
    }
}

fn main() {
    let mut factory = Factory::new();
    factory.run();
}
