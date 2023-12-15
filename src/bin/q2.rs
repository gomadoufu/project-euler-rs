fn main() {
    let result: i64 = (1..40).map_while(fib).filter(|x| x % 2 == 0).sum();
    println!("{:?}", result)
}

fn fib(i: i64) -> Option<i64> {
    match i {
        1 => Some(1),
        2 => Some(2),
        _ => {
            let tmp = fib(i - 1)? + fib(i - 2)?;
            if tmp < 4000000 {
                Some(tmp)
            } else {
                None
            }
        }
    }
}
