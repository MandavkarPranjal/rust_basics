fn main() {
    println!("Hello, world!");

    another_function(6);

    print_measurement(5, 'm');


    // Expressions & Statements
    let y = 6; // statement
    let x = {
        let z = 3;
        z + 1 // expression (no semicolon)
    }; 

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    let a = add_five(2);
    println!("The value of a is: {}", a);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_measurement(length: i32, unit: char) {
    println!("The Lenght is: {}{}", length, unit);
}

fn add_five(x: i32) -> i32 {
    x + 5
}
