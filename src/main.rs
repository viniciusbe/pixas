fn main() {
    let tup = (1, 2, 3);

    let (number, ..) = tup;

    println!("{number}");
}
