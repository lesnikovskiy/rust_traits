mod container;
mod basket;
mod stack;

use container::Container;
use basket::Basket;
use stack::Stack;

fn main() {
    let b1 = Basket::new(String::from("hi there"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let st1 = Stack::new(vec![String::from("hi")]);
    let st2 = Stack::new(vec![1, 2, 3]);
}
