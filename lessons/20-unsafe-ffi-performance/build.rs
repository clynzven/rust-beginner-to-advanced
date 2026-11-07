fn main() {
    cc::Build::new().file("csrc/stats.c").compile("stats");
    println!("cargo:rerun-if-changed=csrc/stats.c");
}
