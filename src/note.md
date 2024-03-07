# Functions
- Functions are prevalent in Rust code
- most important function in Rust is `main`
- `main` is the entry point of many programs
- Rust code uses *snake case* as the conventional style fot Functions and variables, in which all letters are lowercase and underscores separate words. eg: `main.rs` file
- `fn` keyword is used to decalre a function

## Parameters
- use just like every other programming language for Functions

## Statements and Expressions
Function bodies are made up of a series of Statements optionally ending in an expression
- **Statements** are instruction that perform some action and do not return a value
- **Expressions** evaluate to a resulting value
- `let y = 6;` is a statement
- `let x = (let y = 6);` is an error because `let y = 6`is a statement and does not return a value
- `let x = { let y = 6; y + 1};` is an statement but, `y + 1` is a expression because returns a value of `7`

## Return Values
- Rust don't use `return` keyword to return values from function but, declare the return type of the function using `->` followed by the type of the value the function will return 
- In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function
- ou can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly
