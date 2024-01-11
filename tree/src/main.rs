use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<T> {
    value: T,
    // Parent contains a weak reference to avoid cycles that could leak memory
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // 1 strong, 0 weak
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Downgrade Rc to Weak to make the parent reference
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // 1 strong, 1 weak (parent ref in leaf)
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        // 2 strong (+1 from leaf as child ref in branch), 0 weak
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    // Branch goes out of scope: parent ref is None. Since it was weak it allowed
    // branch to be deallocated.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // 1 strong, 0 weak
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
