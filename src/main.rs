use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

struct DoublyLinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    /// Appends an element to the back of a list.
    /// This operation should compute in O(1) time.
    fn push_back(&mut self, elt: T) {
        let mut new_node = Node {
            data: elt,
            next: None,
            prev: None,
        };
        if self.head.is_none() {
            let new_node = Rc::new(RefCell::new(new_node));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            new_node.prev = self.tail.clone();
            let new_node = Rc::new(RefCell::new(new_node));
            if let Some(tail) = &mut self.tail {
                tail.borrow_mut().next = Some(new_node.clone());
            }
            self.tail = Some(new_node);
        }
    }

    /// Adds an element first in the list.
    /// This operation should compute in O(1) time.
    fn push_front(&mut self, elt: T) {
        let mut new_node = Node {
            data: elt,
            next: None,
            prev: None,
        };
        if self.head.is_none() {
            let new_node = Rc::new(RefCell::new(new_node));
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            new_node.next = self.head.clone();
            let new_node = Rc::new(RefCell::new(new_node));
            if let Some(head) = &mut self.head {
                head.borrow_mut().prev = Some(new_node.clone());
            }
            self.head = Some(new_node);
        }
    }
}

impl<T: Clone> fmt::Display for DoublyLinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        write!(f, "head: (")?;
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            // write!(f, " @{:p}", &n.data)?;
            current = n.next.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        write!(f, ") ")?;

        let mut current = self.tail.clone();
        write!(f, "tail: (")?;
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, "{}", n.data)?;
            // write!(f, " @{:p}", &n.data)?;
            current = n.prev.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }
        write!(f, ")")?;

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
