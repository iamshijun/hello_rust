mod state_ds;

use std::ops::Deref;


struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn split_at_mut(slice:&mut[i32],mid:usize) -> (&mut[i32],&mut[i32]) {
    let slice = slice;
    let len = slice.len();
    assert!(mid <= len);

    let ptr = slice.as_mut_ptr();

    let left = unsafe {std::slice::from_raw_parts_mut(ptr,mid)};
    let right = unsafe {std::slice::from_raw_parts_mut(ptr.add(mid),len - mid)};
    (left,right)
}

fn main () {
    let mybox = MyBox::new(String::from("v"));
    println!("{}", mybox.len());

    let x = 5;
    match x {
        1..5 => println!("one to five"),
        _ => println!("something else"),
    }

    let mut slice = vec![1,2,3,4,5];
    let (left,right) = split_at_mut(&mut slice,3);
    println!("left:{:?},right:{:?}",left,right);

}
