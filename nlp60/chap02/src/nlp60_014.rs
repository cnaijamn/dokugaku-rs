fn main() {
    let csv_data = "Apple,100,Red\nBanana,200,Yellow";

    //for line in csv_data.lines() {
    for (idx, line) in csv_data.lines().enumerate() {
        let fields: Vec<&str> = line.split(',').collect();
        let name = fields[0];
        let price = fields[1];
        //println!("商品: {} ({}円)", name, price);
        println!("{}番 商品: {} ({}円)", idx + 1, name, price);
    }
}
