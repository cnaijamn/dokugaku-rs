// https://www.youtube.com/watch?v=sBnDMsHPVf8

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p: Box<Point> = Box::new(Point {
        x: 1,
        y: 2,
    });

    p.x *= 10;
    p.y *= 10;
    println!("p: {} {}", p.x, p.y);
    //-> "p: 10 20"

    let p2: Box<Point> = p;
    println!("p2: {} {}", p2.x, p2.y);
    //-> "p2: 10 20"

    // コンパイルでMoveが発生！
    //println!("p: {} {}", p.x, p.y); //<== エラー

    // 参照
    let p3: &Box<Point> = &p2;
    println!("p2: {} {}", p2.x, p2.y);
    println!("p3: {} {}", p3.x, p3.y);
}
