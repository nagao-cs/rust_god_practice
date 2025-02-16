fn main() {
    let v = [3, 8, 11, 15];
    let mut result = 0;
    for x in v.iter() {
        if *x % 2 == 0 {
            continue;
        }
        result += *x;
    }
    println!("{}", result);
}
