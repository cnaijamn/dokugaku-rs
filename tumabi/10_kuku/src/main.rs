// https://www.youtube.com/watch?v=_DNIOuhY3wc

fn show_kuku(beg: u32, end: u32) {
    for i in beg..end {
        for j in beg..end {
            print!("{:4}", i * j);
        }
        println!("");
    }
}

fn show_char_table(beg: u32, end: u32) {
    for _ in beg..end {
        for j in beg..end {
            let ch = std::char::from_u32(j).unwrap();
            print!("{} ", ch);
        }
        println!("");
    }
}

fn main() {
    /*
    for i in 1..10 {
        for j in 1..10 {
            print!("{:3}", i * j);
        }
        println!("");
    }
    */

    //show_kuku(1, 10);
    show_kuku(5, 15);

    //show_char_table('a' as u32, ('z' as u32) + 1);
    show_char_table('A' as u32, ('z' as u32) + 1);
}
