// キャスト(型変換)

fn main() {
    // 整数型でキャスト
    test_int();

    // 構造体でキャスト
    test_struct();
}

fn test_int() {
    let a: i32 = 100;
    let b: u32 = a as u32;
    println!("{}", b);
    //-> 100

    let a: i32 = -100;
    let b: u32 = a as u32;
    println!("{}", b);
    //-> 4294967196
    // まずい

    // 8bit -> 16bit
    let a: i8 = 8;
    let b: i16 = a as i16;
    println!("{}", b);
    //-> 8

    // 16bit -> 8bit
    let a: i16 = 8;
    let b: i8 = a as i8;
    println!("{}", b);
    //-> 8

    // 二進数を {:b} で表示出力が可能

    let a: i16 = 30000;
    let b: i8 = a as i8;
    println!("{} (a:{:b} b:{:b})", b, a, b);
    //-> 48 (a:111010100110000 b:110000)
    // まずい
}

////////////////////////////////////////////////

// From / Into
// 特定の値を構造体に変換する
use std::convert::{ From, Into };

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i16> for Number {
    fn from(x: i16) -> Self {
        Number {
            value: x as i32,
        }
    }
}

fn test_struct() {
    let a: i16 = 8;
    let num1: Number = Number::from(a);
    let num2: Number = a.into();

    println!("num1: {:?}", num1);
    //-> num1: Number { value: 8 }
    println!("num2: {:?}", num2);
    //-> num1: Number { value: 8 }
}
