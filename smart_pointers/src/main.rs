enum List
{
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

use std::ops::Deref;
impl<T> Deref for MyBox<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl<T> MyBox<T>
{
    fn new(x: T) -> MyBox<T>
    {
        MyBox(x)
    }
}

fn hello(name: &str)
{
    println!("Hello, {}!", name);
}

struct CustomSmartPointer
{
    data: String,
}

impl Drop for CustomSmartPointer
{
    fn drop(&mut self)
    {
        println!("Dropping CustomSmartPointer with data \"{}\"", self.data);
    }
}

enum RcList
{
    RcCons(i32, Rc<RcList>),
    RcNil,
}

use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use std::rc::Rc;

fn main()
{
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let _c = CustomSmartPointer { data: String::from("my stuff") };
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    drop(_c);
    println!("CustomSmartPointer c dropped before the end of main.");

    let _rca = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating rca = {}", Rc::strong_count(&_rca));
    let _rcb = RcCons(3, Rc::clone(&_rca));
    println!("count after creating rcb = {}", Rc::strong_count(&_rca));
    {
        let _rcc = RcCons(4, Rc::clone(&_rca));
        println!("count after creating rcc = {}", Rc::strong_count(&_rca));
    }
    println!("count after rcc goes out of scope = {}",
             Rc::strong_count(&_rca));
}
