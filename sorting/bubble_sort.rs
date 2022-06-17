// Sort the vector in ascending order using optimized bubble sort

use std::io;

fn bubble_sort<T: PartialOrd>(vec: &mut [T]) {
    // Length of the vector
    let len = vec.len();

    // Loop to traverse the vector
    for i in 0..len - 1 {
        let mut swapped = 0;

        // Loop to compare the elements
        for j in 0..len - i - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                swapped = 1;
            }
        }

        // Check if the elements are sorted
        if swapped == 0 {
            break;
        }
    }
}

// driver code
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

    println!("\nEnter the values of the vector: ");

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

    println!("\nUnsorted vector: {:?}", vec);
    bubble_sort(&mut vec);
    println!("\nThe sorted vector is: {:?}", vec);
}