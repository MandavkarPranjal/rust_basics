# Ownership
- *Ownership* is a set of rules that govern how a Rust program can use memory safely and efficiently
- Ownership is Rust's most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector
- Many other languages with garbage collection have to make trade-offs between the safety and predictability of the program and the cost of running it 
- All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory
- Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile
- None of the ownership features slow your program while it's running

## The Stack and The Heap
- Rust is a systems programming language, so whether a value is stored on the stack or the heap affects how the language behaves and is something need to be aware of 
- Both the stack and the heap are parts of memory that are available to code at runtime
- The stack follows the ***Last In, First Out(LIFO)*** 
- The heap is less organized: when you put data on the heap, you request a certain amount of space from space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a *pointer*, which is the address of that location. This process is called ***allocating on the Heap*** and is sometimes abbreviated as just ***allocating*** 
- *pointer* is a known, fixed-size piece of data, and you can store the pointer on the stack, but when you want the actual data, you must follow the pointer
- Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack
> [!IMPORTANT]
> All the data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead

## Ownership Rules
-> [!IMPORTANT]
> 1. Each value in Rust has an *owner* 
> 2. There can only be one owner at a time 
> 3. When the owner goes out of scope, the value will be dropped

## Variable Scope
- A variable is valid from the point at which it is declared until the end of the current scope
```Rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }
```
- when ```s``` comes into scope, it is valid
- It remains valid until it goes out of scope

## The String Type
```Rust
let s = String::from("hello");
// the double colon (::) is an operator that allows us to namespace this particular *from* function under the *String*  type rather than using some sort of name like *string_from* 
```
- This kind of string can be mutated
```Rust
let mut s = String::from("hello");

s.push_str(" world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello world!`
```
