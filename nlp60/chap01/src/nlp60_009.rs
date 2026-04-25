fn main() {
    let nums = [10, 20, 30, 40, 50];
    let mut sum = 0;

    for n in nums {
        sum += n;
    }

    println!("合計: {}", sum);
}
