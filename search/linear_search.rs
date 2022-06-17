use std::io;

fn linear_search<T: PartialEq> (v: &[T], value: &T) -> i32 {
    let mut pos = -1;
    for (index, data) in v.iter().enumerate() {
        if value == data {
            pos = index as i32;
            return pos;
        }
    }
    pos
}

// Driver code
fn main() {
    let mut vec = Vec::new();

    println!("Enter the size of the vector: ");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n = n
        .trim()
        .parse::<i32>()
        .expect("Invalid integer input");

    println!("Enter the values of the vector: ");

    for _ in 0..n  {
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");

        let value = value
            .trim()
            .parse::<i32>()
            .expect("Invalid input");

        vec.push(value);
    }

    println!("Vector: {:?}", vec);

    println!("\nEnter the element you want to search for in the vector: ");
    let mut element = String::new();
    io::stdin()
        .read_line(&mut element)
        .expect("Failed to read line");

    let element = element
        .trim()
        .parse::<i32>()
        .expect("Invalid input");

    let position = linear_search(&vec, &element);

    println!("\nThe element is found at index: {}", position);
}