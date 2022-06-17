// Sort the vector in ascending order

use std::io;

fn partition<T: PartialOrd>(vec: &mut [T]) -> usize {
    let mut i = 0;
    let mut j = 0;
    let len = vec.len();

    while j < len - 1 {
        if vec[j] <= vec[len - 1] {
            vec.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    vec.swap(i, len - 1);
    i
}

fn quick_sort<T: Ord>(v: &mut [T]) {
    if !v.is_empty() {
        let mid = partition(v);
        let len = v.len();
        
        //left of the pivot
        quick_sort(&mut v[0..mid]);

        //right of the pivot
        quick_sort(&mut v[mid + 1..len]);
    }
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
    quick_sort(&mut vec);
    println!("\nSorted vector: {:?}", vec);
}