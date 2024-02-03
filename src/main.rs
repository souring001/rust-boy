use std::fmt;

#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

struct DoublyLinkedList<T: Clone> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T: Clone> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    /// Removes the last element from a list and returns it, or None if it is empty.
    /// This operation should compute in O(1) time.
    fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
    }

    /// Removes the first element and returns it, or None if the list is empty.
    /// This operation should compute in O(1) time.
    fn pop_front(&mut self) -> Option<T> {
        unimplemented!()
    }

    /// Appends an element to the back of a list.
    /// This operation should compute in O(1) time.
    fn push_back(&mut self, elt: T) {
        let new_node = Node {
            data: elt,
            next: None,
            prev: None,
        };
        self.head = Some(Box::new(new_node.clone()));
        self.tail = Some(Box::new(new_node));
    }

    /// Adds an element first in the list.
    /// This operation should compute in O(1) time.
    fn push_front(&mut self, elt: T) {
        unimplemented!()
    }
}

impl<T: Clone> fmt::Display for DoublyLinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        write!(f, "(");
        while let Some(node) = current {
            let n = node;
            write!(f, "{}", n.data)?;
            current = n.next.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        write!(f, ")");
        Ok(())
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    println!("{}", list); // ()

    list.push_back(1);

    println!("{}", list); // (1)

    list.push_back(2);
    list.push_back(3);

    println!("{}", list); // (1<--->2<--->3)

    list.push_front(4);
    list.push_front(5);

    println!("{}", list); // (5<--->4<--->1<--->2<--->3)
}
