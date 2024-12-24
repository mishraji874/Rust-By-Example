fn main() {
    // by default variables are immutable in rust
    let x = 5;
    println!("The value of x is: {}", x);

    // to make the variable mutable -> i.e we can change the value of variable, we can use the `mut` keyword
    let mut y = 5;
    println!("The value of y is: {}", y);

    // now we can reassign the variable y because we have declared it as mutable variable
    y = 6;
    println!("The value of y is: {}", y);

    // in rust, types can be inferred i.e. they are automatically assigned by the rust compiler
    let z = 5; // z: i32
    println!("The value of z is: {}", z);

    // or we can declare the types by ourselves also
    let a: i32 = 5;
    println!("The value of a is: {}", a);

    // Variables can be shadowed, the variable a is redeclared with a different type, and the value is reassigned.
    let a = "hello, world";
    println!("The value of a is: {}", a);
}