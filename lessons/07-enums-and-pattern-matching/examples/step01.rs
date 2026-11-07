#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn main() {
    let mut d = Direction::North;
    for _ in 0..5 {
        print!("{d:?} -> ");
        d = turn_right(d);
    }
    println!("{d:?}");
}
