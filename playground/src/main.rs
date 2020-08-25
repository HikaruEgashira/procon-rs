#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn include3(x: i32, i: i32) -> Option<i32> {
    if x % 10 == 3 {
        Some(i)
    } else {
        if x > 10 {
            include3(x / 10, i)
        } else {
            None
        }
    }
}

fn main() {
    input! {
        n: i32
    }

    let mut arr = vec![];

    for i in 1..n + 1 {
        if i % 3 == 0 {
            arr.push(Some(i));
        } else {
            arr.push(include3(i, i));
        }
    }

    println!(
        " {}",
        arr.iter()
            .filter(|a| a.is_some())
            .map(|a| a.unwrap().to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
