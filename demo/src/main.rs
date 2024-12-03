use core::prelude;
use std::arch::x86_64::_MM_ROUND_TOWARD_ZERO;
use std::cell::RefCell;
use std::ffi::FromBytesUntilNulError;
use std::rc::{self, Rc, Weak};

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

    pub fn push_back(&mut self, value:T){
        let new_node=Node::new(value);
        match self.tail.take(){
            Some(old_tail)=>{
                if let Some(prev_tail)=old_tail.upgrade(){
                    prev_tail.borrow_mut().next=Some(new_node.clone());
                    new_node.borrow_mut().prev=Some(old_tail);
                }
                self.tail=Some(Rc::downgrade(&new_node));
            }
            Node=>{
                self.head=Some(new_node.clone());
                self.tail=Some(Rc::downgrade(&new_node));
            }
        }
    }
    
    pub fn pop_front(&mut self)->Option<T>{
        self.head.take().map(|head|{
            if let Some(next) = head.borrow_mut().next.take(){
                next.borrow_mut().prev=None;
                self.head=Some(next);
            } else {
                self.tail.take();
            }
            Rc::try_unwrap(head).ok().unwrap().into_inner().value
        })
    }

    pub fn peek_front(&self)->Option<Rc<T>>{
        self.head
            .as_ref()
            .map(|head| Rc::clone(&head.borrow().value))
    }

}
fn main(){
    let mut 
    list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.peek_front(3);

    if let Some(front_value) = list.peek_front(){
        println!("front of the list:{:?}",front_value);
    } else{
        println!("list is empty.");
    } 
    println!("pop front :{:?}",list.pop_front());
    if let Some(front_value)=list.peek_front(){
        println!("front of the list after pop:{:?}",front_value);
    } else {
        println!("list is empty after pop.");
    }
pub fn pop_front(&mu)