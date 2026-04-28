use rand::RngExt;

fn main() {
    // 0:グー 1:チョキ 2:パー
    let hands = ["グー", "チョキ", "パー"];
    let my_hand = 0; // 自分
    let pc_hand = rand::rng().random_range(0..3); // 相手

    println!("自分: {}, 相手: {}", hands[my_hand], hands[pc_hand]);

    /*
    if my_hand == pc_hand {
        println!("あいこ");
    } else if (my_hand == 0 && pc_hand == 1)
        || (my_hand == 1 && pc_hand == 2)
        || (my_hand == 2 && pc_hand == 0)
    {
        println!("勝ち！");
    } else {
        println!("負け...");
    }
    */

    /*
    match (my_hand, pc_hand) {
        (my, pc) if my == pc => println!("あいこ"),
        (0, 1) | (1, 2) | (2, 0) => println!("勝ち！"),
        _ => println!("負け..."),
    }
    */

    let result = match (my_hand, pc_hand) {
        (my, pc) if my == pc => format!("あいこ"),
        (0, 1) | (1, 2) | (2, 0) => format!("勝ち！"),
        _ => format!("負け..."),
    };
    println!("{}", result);
}
