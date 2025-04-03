fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.pop();
    v.pop();

    println!("{:?}", v);
}