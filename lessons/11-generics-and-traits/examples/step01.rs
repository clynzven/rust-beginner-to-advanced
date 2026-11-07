fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest_i32(&[3, 9, 2]));
    println!("{}", largest(&[3, 9, 2]));
    println!("{}", largest(&[1.5, 0.2, 7.25]));
    println!("{}", largest(&['r', 'u', 's', 't']));
}
