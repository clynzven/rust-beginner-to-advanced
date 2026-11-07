use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Account {
    owner: String,
    balance: i64,
}

fn main() {
    let shared = Rc::new(RefCell::new(Account { owner: "Ana".into(), balance: 100 }));

    let mobile_app = Rc::clone(&shared);
    let web_app = Rc::clone(&shared);

    mobile_app.borrow_mut().balance -= 30;
    web_app.borrow_mut().balance += 50;

    let acct = shared.borrow();
    println!("{} has {} (shared by {} owners)", acct.owner, acct.balance, Rc::strong_count(&shared));
}
