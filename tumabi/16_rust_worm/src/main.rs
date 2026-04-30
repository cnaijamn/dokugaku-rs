use std::io::Write; // flush()

const MSEC: u64 = 200;

enum Mode {
    First,
    Second,
}

fn put_spaces() {
    //for _ in 0..10 {
    //    print!(" ")
    //}
    put_x_spaces(10);
}

fn put_x_spaces(x: i32) {
    for _ in 0..x {
        print!(" ")
    }
}

fn main() {
    let mut cnt: i32 = 1;
    let mut m = Mode::First;
    let mut x: i32 = 0;

    loop {
        print!("\r");
        match m {
            Mode::First => {
                put_x_spaces(x);
                if cnt % 2 == 0 {
                    print!("＿＿(・ω・)"); // '\r': CR (Carriage Return)
                } else {
                    print!("  ＿|\\(・ω・)"); // '\r': CR (Carriage Return)
                }
                put_spaces();
                if cnt % 20 == 0 {
                    m = Mode::Second;
                }
                x += 1;
            }
            Mode::Second => {
                put_x_spaces(x);
                if cnt % 2 == 0 {
                    print!("(・ω・)＿＿"); // '\r': CR (Carriage Return)
                } else {
                    print!("  (・ω・)/|＿"); // '\r': CR (Carriage Return)
                }
                put_spaces();
                if cnt % 20 == 0 {
                    m = Mode::First;
                }
                x -= 1;
            }
        }

        if cnt % 40 == 0 {
            println!("");
        }

        cnt += 1;
        std::io::stdout().flush().expect("failed to flush");
        std::thread::sleep(std::time::Duration::from_millis(MSEC));
    }
}
