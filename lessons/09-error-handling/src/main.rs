use anyhow::{Context, Result, bail};

fn load_port(path: &str) -> Result<u16> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading {path}"))?;
    let port: u16 = text.trim().parse()
        .context("port must be a number")?;
    if port < 1024 {
        bail!("port {port} is reserved, use 1024 or higher");
    }
    Ok(port)
}

fn main() {
    std::fs::write("good.txt", "8080").unwrap();
    std::fs::write("low.txt", "80").unwrap();
    std::fs::write("bad.txt", "http").unwrap();
    for f in ["good.txt", "low.txt", "bad.txt", "missing.txt"] {
        match load_port(f) {
            Ok(p) => println!("{f}: port {p}"),
            Err(e) => println!("{f}: {e:#}"),
        }
    }
}
