fn main() {
    //variables can always be type annotated
    let logical: bool = true; // true or false

    // floating point: f32, f64
    let a_float: f64 = 1.0; // Regular annotation
    let b_float = 1.0f32; // Suffix annotation
    
    // Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    let an_integer = 5i32; // Suffix annotation
    let bn_integer: i32 = 5; // Regular annotation
    
    // There are also unsigned integers: u8, u16, u32m u64, u128 and usize (pointer size)
    let a_unsigned = 5u16;
    let b_unsigned: u128 = 5;

    // default types are i32 and f64
    let default_float = 3.0; // `f64`
    let default_integer = 13; // `i32`

    // by default variables are immutable
    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21;

    // A type can also be inferred from context
    let mut inferred_type = 5; // Type i64 is inferred from another line 
    inferred_type = 4294967296i64;

    // Error! The type of a variables can't be changed
    mutable = true;

    // variables can be overwritten with shadowing
    let _mutable = true;
}
