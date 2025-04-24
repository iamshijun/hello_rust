use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

#[test]
fn test_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    // leaf.parent.borrow().upgrade(); // => Option<Rc<Node>>

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong count {:?}", Rc::strong_count(&leaf));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);//downgrade 会返回一个Weak

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}