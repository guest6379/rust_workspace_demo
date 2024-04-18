use std::{
    fmt::Display,
    ops::Deref,
};

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping ... {}", &self.0);
    }
}

struct Person {
    name: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}
fn main() {
    println!("{:}", "r1 in rec");

    {

        let x = Person {
            name: "bill".to_string(),
        };
        let y = MyBox::new(x);
        println!("{:}",*y);
        std::mem::drop(y);
        println!("{:}","kkkkkkkkkkkkkkkkkkk");
    }


    println!("{:}","????????");
}
