- by default, variables are immutable
- when a variables is immutable, once a value is bound to a name, you can't change that value.
- to make a variable mutable, use the `mut` keyword.
- `mut` is a short form of mutable.

## Constants
" Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
- constants aren't allowed to use the `mut` keyword
- constants aren't just immutable by default, they are always immutable
- constants are declared using the `const` keyword and the type of the value must be annotated
- constants can be declared in any scope, including the global scope, which make them useful for values that many parts of the program need to know about
- last difference is that constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime

- constants are valid for the entire time a program runs, within the scope they were declared

## Shadowing
- Shadowing is a feature that allows you to redeclare a variables
- Shadowing is different from marking a variable as `mut`.
- Shadowing is useful when you want to change the type of a value but reuse the same name
- Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable
