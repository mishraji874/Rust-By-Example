// Closures are anonymous functions that can capture variables from their surrounding environment. 
// Closures can be stored as variables and passed around, they can also be used as arguments to functions. 
// They are declared using the || syntax.

fn main() {
    let addition_v1 = |a: i32, b: i32| -> i32 { a + b };

    println!("Adding two numbers: {}", addition_v1(1, 2));

    let addition_v2 = |c: i32, d: i32| c + d;
    println!("Adding two numbers: {}", addition_v2(2, 3));

    let addition_v1 = |e, f| e + f;
    println!("Adding two numbers: {}", addition_v1(3, 4));

    let x = 1;
    let capture_x = |y| x + y;
    println!("Capturing x: {}", capture_x(2));

    fn call_with_one<F>(closure: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        closure(1)
    }

    let answer = call_with_one(|x| x + 2);
    println!("Answer: {}", answer);
}