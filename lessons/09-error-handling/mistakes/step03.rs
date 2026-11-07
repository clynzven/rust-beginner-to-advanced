fn parse_pair(s: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    let (a, b) = s.split_once(',').unwrap_or((s, ""));
    Ok((a.trim().parse()?, b.trim().parse()?))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (x, y) = parse_pair("3, 4")?;
    println!("x={x} y={y}");
    let text = std::fs::read_to_string("nope.txt")?;
    println!("{text}");
    Ok(())
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling errors v0.1.0 (~/rust-course/errors)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
//      Running `target/debug/errors`
// x=3 y=4
// Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
