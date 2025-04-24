use std::cell::RefCell;


use crate::list::boxlist::BoxList::{Cons,Nil};

#[derive(Debug)]
enum BoxList<T> {
    Cons(T, RefCell<Box<BoxList<T>>>),
    Nil
}
impl<T> BoxList<T> {
    fn tail(&self) -> Option<&RefCell<Box<BoxList<T>>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}