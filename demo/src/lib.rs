use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 链表节点
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}

// 双链表
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        mate() {
            Some(old_tail) => {
                if let Some(prev_tail) = old_tail.upgrade() {
                    prev_tail.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(old_tail);
                }
                self.tail = Some(Rc::downgrade(&new_node));
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            Rc::try_unwrap(head).ok().unwrap().into_inner().value
        })
    }

    pub fn peek_front(&self) -> Option<Rc<T>> {
        self.head
            .as_ref()
            .map(|head| Rc::clone(&head.borrow().value))
    }
}

fn main() {    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    if let Some(front_value) = list.peek_front() {
        println!("Front of the list: {:?}", front_value);
    } else {
        println!("List is empty.");
    }

    println!("Pop front: {:?}", list.pop_front());
    if let Some(front_value) = list.peek_front() {
        println!("Front of the list after pop: {:?}", front_value);
    } else {
        println!("List is empty after pop.");
    }
}
        