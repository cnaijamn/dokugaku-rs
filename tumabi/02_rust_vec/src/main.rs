// https://www.youtube.com/watch?v=qsoxP_4PBqQ

fn main() {
    {
        let v: Vec<i32> = Vec::new();
        println!("{}", v.len());
        //-> 0

        let v = vec![0; 0]; // 型判別
        println!("{}", v.len());
        //-> 0

        let v = vec![10, 20, 30];
        println!("{}", v.len());
        //-> 3
        println!("{}", v[1]);
        //-> 20
        println!("{:?}", v);
        //-> [10, 20, 30]

        let v = vec![10; 3];
        println!("{:?}", v);
        //-> [10, 10, 10]
    }

    { // POPした値にunwrapが必要
        let mut v = vec![10, 20, 30];
        v.push(40);
        println!("{} {:?}", v.len(), v);
        //-> 4 [10, 20, 30, 40]

        let popped = v.pop();
        println!("{} {:?}", v.len(), v);
        //-> 3 [10, 20, 30]
        println!("popped = {}", popped.unwrap());
        //-> popped = 40

        let mut v = vec![0; 0]; // 空
        println!("{} {:?}", v.len(), v);
        //-> 0 []
        let popped = v.pop();
        //エラー(パニック)
        //println!("popped = {}", popped.unwrap());
        //-> thread 'main' (14900) panicked at src/main.rs:41:40:
        //   called `Option::unwrap()` on a `None` value
        //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        if popped == None { // チェックが必要
            // エラー
            println!("None!!!");
            //-> None!!!
        }
    }

    {
        let v = vec![10, 20, 30];

        for el in v.iter() {
        //for el in v { // これでもOK
            println!("el = {}", el);
        }
        //-> el = 10
        //   el = 20
        //   el = 30

        // 添え字を追加
        for (i, el) in v.iter().enumerate() { // 添え字 i
            println!("[{}] el = {}", i, el);
        }
        //-> [0] el = 10
        //   [1] el = 20
        //   [2] el = 30

        // C言語的(柔軟性？)
        let v = vec![100, 200, 300];

        for i in 0..v.len() {
            println!("[{}] el = {}", i, v[i]);
        }
        //-> [0] el = 100
        //   [1] el = 200
        //   [2] el = 300
    }
}
