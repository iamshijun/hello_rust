use std::cell::RefCell;

use crate::list::reflist::RefList::{Cons,Nil};

#[derive(Debug)]
enum RefList<'a, T> {
    Cons(T, RefCell<&'a RefList<'a,T>>),
    Nil
}
impl<'a,T> RefList<'a, T> {
    fn tail(&'a self) -> Option<&'a RefCell<&'a RefList<'a,T>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}


#[test]
fn test_list() {
    let a = Cons(3, RefCell::new(&Nil));
    let b = Cons(4, RefCell::new(&a));

    let c = Cons(5, RefCell::new(&Nil));
    
    println!("before {:?}", b);
    //RefCell 让我们可以在不可变引用的情况下修改值
    *b.tail().unwrap().borrow_mut() = &c;

    //*c.tail().unwrap().borrow_mut() = &b;

    println!("after {:?}", b);

    let next = Cons(2, RefCell::new(&b));
    let list = Cons(1, RefCell::new(&next)); // 1 -> 2 -> b

    let mut tail = list.tail();
    while let Some(t) = tail {
        match *t.borrow() {
            Cons(value, n) => {
                println!("{}", value);
                tail = Some(n)
            },
            Nil => break
        }
        //tail = t.borrow().tail();
    }
}