fn bubble_sort(v: &mut Vec<i32>, compar: fn(i32, i32) -> bool) {
    let mut flag = true;
    let mut k = 0;

    while flag {
        flag = false;
        let mut i = 0;

        while i < v.len() - 1 - k {
            if compar(v[i], v[i + 1]) {
                let tmp = v[i];
                v[i] = v[i + 1];
                v[i + 1] = tmp;
                flag = true;
            }

            i += 1;
        }

        k += 1;
    }    
}

fn desc(a: i32, b: i32) -> bool {
    a < b
}

fn asc(a: i32, b:i32) -> bool {
    a > b
}

fn show_vec(v: &Vec<i32>) {
    for el in v.iter() {
        println!("{}", el);
    }
}

fn line(s: &str, n: i32) {
    for _ in 0..n {
        print!("{}", s);
    }
    println!("");
}

fn main() {
    let mut v = vec![5, 2, 3, 1, 4];

    bubble_sort(&mut v, desc);
    show_vec(&v);
    //-> 5
    //   4
    //   3
    //   2
    //   1

    line("-", 40);

    bubble_sort(&mut v, asc);
    show_vec(&v);
    //-> 1
    //   2
    //   3
    //   4
    //   5
}
