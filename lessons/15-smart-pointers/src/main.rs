use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    name: String,
    parent: RefCell<Weak<Node>>,         // weak: does not keep the parent alive
    children: RefCell<Vec<Rc<Node>>>,    // strong: owns the children
}

fn node(name: &str) -> Rc<Node> {
    Rc::new(Node { name: name.into(), parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) })
}

fn main() {
    let root = node("root");
    let leaf = node("leaf");
    *leaf.parent.borrow_mut() = Rc::downgrade(&root);
    root.children.borrow_mut().push(Rc::clone(&leaf));

    let parent_name = leaf.parent.borrow().upgrade().map(|p| p.name.clone());
    println!("leaf's parent = {parent_name:?}");
    println!("root: strong={} weak={}", Rc::strong_count(&root), Rc::weak_count(&root));
    println!("leaf: strong={} weak={}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("root has {} child", root.children.borrow().len());

    drop(root);
    println!("after dropping root, parent = {:?}", leaf.parent.borrow().upgrade().map(|p| p.name.clone()));
}
