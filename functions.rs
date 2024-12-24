fn main() {
    fn add(x: i32, y: i32) -> i32 {
        return x + y;
    }

    let result = add(1, 2);
    println!("Adding two numbers: {}", result);

    fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }

    let result = subtract(2, 1);
    println!("subtracting two numbers: {}", result);

    fn swap(x: i32, y: i32) -> (i32, i32) {
        return (y, x);
    }

    let result = swap(1, 2);
    println!("Swapping two numbers: {} {}", result.0, result.1);

    fn no_return() {}

    let result = no_return();
    println!("No return value: {:?}", result);
}