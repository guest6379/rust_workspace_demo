use std::rc::Rc;

fn main() {
    println!("{:}", "r2");

    let a = 5;
    let rc1 = Rc::new(&a);
    {
        let rc1 = Rc::clone(&rc1);


        println!("{:}",Rc::strong_count(&rc1));
    }
    println!("{:}",Rc::strong_count(&rc1));
}
