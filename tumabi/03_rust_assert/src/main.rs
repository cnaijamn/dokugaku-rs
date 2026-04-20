// https://www.youtube.com/watch?v=ZYo9RXZtwrI
/*
    - assert!(bool)
    - assert_eq!(T, T)
    - assert_ne!(T, T)
*/

fn main() {
    { // assert!(bool)
        assert!(true); // OK

        //assert!(false); // NG (パニック)
        //-> thread 'main' (23255) panicked at src/main.rs:11:5:
        //   assertion failed: false
        //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        let x = 0;
        assert!(x == 0, "x is zero"); // OK

        //let x = 999;
        //assert!(x == 0, "x is zero"); // NG
        //-> thread 'main' (24804) panicked at src/main.rs:19:5:
        //   x is zero
        //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    }

    { // assert_eq!(第一引数, 第二引数)
        let x = 0;
        assert_eq!(x, 0); // OK

        let x = "aa";
        assert_eq!(x, "aa"); // OK

        //let x = "xx";
        //assert_eq!(x, "aa"); // NG
        //-> thread 'main' (26323) panicked at src/main.rs:29:9:
        //   assertion `left == right` failed
        //     left: "xx"
        //    right: "aa"
        //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

        //let x = "xx";
        //assert_eq!(x, "aa", "x is \"aa\""); // NG
        //-> thread 'main' (27730) panicked at src/main.rs:43:9:
        //   assertion `left == right` failed: x is "aa"
        //     left: "xx"
        //    right: "aa"
        //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    }

    { // assert_ne!(第一引数, 第二引数)
        let x = 0;
        assert_ne!(x, 100); // OK

        //let x = 100;
        //assert_ne!(x, 100); // NG
        //-> thread 'main' (28472) panicked at src/main.rs:56:9:
        //   assertion `left != right` failed
        //     left: 100
        //    right: 100
        //   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    }

    println!("All tests are OK.");
}
