fn main() {
    let value = 1;
    match value {
        // match with a single pattern
        1 => println!("one"),

        // match with multiple patterns
        2 | 3 => println!("two or three"),

        // match with a range
        2..5 => println!("two through five"),

        // match with a guard.
        // a guard is an additional condition that must be true for pattern to match
        x if x > 5 => println!("greater than five"),

        // match with the binding `@`
        x @ 12..20 => println!("twelve through twenty: {}", x),

        // match with wildcard i.e. it can match with anyone
        _ => println!("something else"),
    }
}