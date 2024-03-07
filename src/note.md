# Control Flow
This is a fancy name for `if` expressions and loops

### `if` Expressions
- same as **cpp**
```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### Using `if` in a `let` Statement
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```
## Repetition with loops
Rust has three kinds of loops:
- `loop`
- `while`
- `for`

### `loop`
- The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
- `break` keyword to stop the loop
- `continue` keyword to skip the rest of the iteration and start the next one
- > [!IMPORTANT]
> Rust also has a `return` keyword to return a value from a loop
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```
**Loop Labels to Disambiguate Between Multiple Loops** 
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

### `while`
- The `while` loop is a conditional loop: it runs as long as a condition holds
> [!NOTE]
> I don't remember cpp `while` loop, but Rust `while` loop is similar to python `while` loop
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

### `for`
> [!NOTE]
> `for` loop in Rust is similar to python `for` loop
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
