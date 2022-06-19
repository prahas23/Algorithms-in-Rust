// Implementation of singly linked list

use std::fmt::{Display, Formatter, Result};

// Creates a generic struct Node
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Generic linked list
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.next?;
        self.next = node.next.as_deref();
        Some(&node.data)
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn insert_front(&mut self, data: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node{ data, next }));
    }

    pub fn delete_front(&mut self) -> Option<T> {
        let head = self.head.take()?;
        self.head = head.next;
        Some(head.data)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

// Implements display for the linked list
impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for node in self.iter() {
            write!(f, "{} -> ", node)?
        }
        Ok(())
    }
}

// Driver code
fn main() {
    let mut list = List::new();

    for i in 0..5 {
        list.insert_front(i);
    }

    println!("Linked list: {}", list);

    let popped = list.delete_front();
    println!("Deleted element is: {:?}", popped.unwrap());

    println!("Updated linked list: {}", list);
}