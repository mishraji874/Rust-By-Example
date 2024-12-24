fn main() {
    // `loop` keyword tells compiler to execute a block of code over and over forever, until you stop it
    loop {
        println!("hello, world!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    // while loop runs as long as condition holds true
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    // for loop is used to iterate through a collections of items
    let array = [1, 2, 3, 4, 5];
    for element in array {
        println!("{}", element);
    }

    for i in 0..5 {
        println!("{}", i);
    }

    for i in 0..=5 {
        println!("{}", i);
    }
}