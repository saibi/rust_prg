use std::ops::{Deref, DerefMut};

struct MySmartPointer<T>(T);

impl<T> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer");
    }
}

fn main() {
    let my_pointer = MySmartPointer::new(5);
    println!("value: {}", *my_pointer);

    let mut mut_pointer = MySmartPointer::new(1);
    *mut_pointer = 2;
    println!("value: {}", *mut_pointer);
}
