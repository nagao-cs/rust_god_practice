mod sort_func;

fn main() {
    let v = vec![2, 4, 5, 1];
    sort_func::selection_sort(v);
    println!("{}", v);
}
