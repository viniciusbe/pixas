fn functions() {
    let x = 5;
    let y = add_one(x);
    println!("X value:{x}");
    println!("Y add one:{y}");
}

//parameters and return types are mandatory
fn add_one(x: i32) -> i32 {
    println!("The parameter value is: {x}");
    let y = 1; // ends with a semicolon: is a statement, is a action and has no return
    x + 1 // no semicolon: is a expression, implicit return
}
