fn main() {
    if true {
        println!("true");
    } else if false {
        println!("false");
    } else {
        println!("neither");
    }

    let n = 42;
    if n > 0 && n < 100 {
        println!("n is between 0 and 100");
    } else if n < 0 || n > 100 {
        println!("n is not between 0 and 100");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let result: Option<i32> = Some(42);
    if let Some(value) = result {
        println!("The value is: {}", value);
    } else {
        println!("There is no value");
    }
}