fn main() {
    _for();
}

fn _if() {
    let condition = true;
    let number = if condition { 5 } else { 7 };
    println!("{number}");
}

fn _loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("{result}")
}

fn _while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("end")
}

fn _for() {
    let array = [1, 2, 3, 4, 5];

    for el in array {
        println!("{el}");
    }
}
