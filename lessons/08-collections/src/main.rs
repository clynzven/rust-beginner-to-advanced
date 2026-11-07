use std::collections::{BTreeMap, HashSet, VecDeque};

fn main() {
    let mut grades = BTreeMap::new();          // sorted by key
    grades.insert("zoe", 91);
    grades.insert("ana", 85);
    grades.insert("max", 78);
    println!("{grades:?}");

    let a: HashSet<i32> = [1, 2, 3, 4].into();
    let b: HashSet<i32> = [3, 4, 5].into();
    let mut both: Vec<_> = a.intersection(&b).collect();
    both.sort();
    println!("in both: {both:?}");

    let mut queue = VecDeque::from([1, 2, 3]);
    queue.push_front(0);
    queue.push_back(4);
    println!("front={:?} queue={queue:?}", queue.pop_front());
}
