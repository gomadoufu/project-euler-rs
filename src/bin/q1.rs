fn main() {
    let sum = (0..1000)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!("sum = {}", sum);
}
