fn main() {
    let num1 = 10.0;
    let num2 = 5.0;
    let operator = "+";

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => 0.0,
    };

    println!("{} {} {} = {}", num1, operator, num2, result);
}
