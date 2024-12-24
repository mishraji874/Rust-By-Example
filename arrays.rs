// The array type is a compound type that allows you to store multiple values of the same type next to each other in memory. 
// Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements.

fn main() {
    let arr_inferred = [1, 2, 3, 4, 5];
    let arr_explicit: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr_inferred: {:?}", arr_inferred);
    println!("arr_explicit: {:?}", arr_explicit);

    let arr_same = [3; 5];
    println!("arr_same: {:?}", arr_same);

    let arr_2d: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("arr_2d: {:?}", arr_2d);

    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("first: {}, second: {}", first, second);

    let index: usize = 2;
    let third = arr[index];
    println!("third: {}", third);

    // arrays are immutable by default so we have to use `mut` for changing the values in the array
    let mut arr = [1, 2, 3, 4, 5];
    arr[0] = 10;
    println!("arr: {:?}", arr);

    let size = arr_inferred.len();
    println!("size: {}", size);

    let slice = &arr[1..3];
    let rest = &arr[1..];
    let all = &arr[..];
    println!("slice: {:?}", slice);
    println!("rest: {:?}", rest);
    println!("all: {:?}", all);

    for element in arr.iter() {
        println!("element: {}", element);
    }

    for (i, element) in arr.iter().enumerate() {
        println!("index: {}, element: {}", i, element);
    }

    for i in 0..arr.len() {
        println!("index: {}, element: {}", i, arr[i]);
    }
}