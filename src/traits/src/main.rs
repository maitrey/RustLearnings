mod basket;
mod stack;
mod container;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

fn main() {
    let b1 = Basket::new(String::from("Hello!"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("Hello")]);
    let s2 = Stack::new(vec![2, 4,6 ]);
    
    add_string(&mut s1, String::from("World!"));
}
