# Rust: Beginner to Advanced

The source code for the free 24-lesson video course **Rust: Beginner to Advanced** on the
**ClynzVen Code Academy** YouTube channel.

▶️ **Watch the course:** [youtube.com/@ClynzVen/playlists](https://www.youtube.com/@ClynzVen/playlists)

Every lesson's code is here and runs: the final program, every version the video builds
along the way, and the versions it breaks on purpose to read the compiler's errors.
Tested with Rust 1.97 (edition 2024) on Linux; it works the same on macOS and Windows.

## The lessons

| # | Lesson | Code | Part |
|---|---|---|---|
| 1 | Hello, Rust | [`lessons/01-hello-rust`](lessons/01-hello-rust) | Beginner |
| 2 | Variables and Types | [`lessons/02-variables-and-types`](lessons/02-variables-and-types) | Beginner |
| 3 | Functions and Control Flow | [`lessons/03-functions-and-control-flow`](lessons/03-functions-and-control-flow) | Beginner |
| 4 | Ownership | [`lessons/04-ownership`](lessons/04-ownership) | Beginner |
| 5 | Borrowing and Slices | [`lessons/05-borrowing-and-slices`](lessons/05-borrowing-and-slices) | Beginner |
| 6 | Structs and Methods | [`lessons/06-structs-and-methods`](lessons/06-structs-and-methods) | Beginner |
| 7 | Enums and Pattern Matching | [`lessons/07-enums-and-pattern-matching`](lessons/07-enums-and-pattern-matching) | Beginner |
| 8 | Collections: Vec, String and HashMap | [`lessons/08-collections`](lessons/08-collections) | Intermediate |
| 9 | Error Handling | [`lessons/09-error-handling`](lessons/09-error-handling) | Intermediate |
| 10 | Modules, Crates and Cargo | [`lessons/10-modules-and-cargo`](lessons/10-modules-and-cargo) | Intermediate |
| 11 | Generics and Traits | [`lessons/11-generics-and-traits`](lessons/11-generics-and-traits) | Intermediate |
| 12 | Lifetimes | [`lessons/12-lifetimes`](lessons/12-lifetimes) | Intermediate |
| 13 | Closures and Iterators | [`lessons/13-closures-and-iterators`](lessons/13-closures-and-iterators) | Intermediate |
| 14 | Testing and Documentation | [`lessons/14-testing-and-documentation`](lessons/14-testing-and-documentation) | Intermediate |
| 15 | Smart Pointers | [`lessons/15-smart-pointers`](lessons/15-smart-pointers) | Advanced |
| 16 | Fearless Concurrency | [`lessons/16-concurrency`](lessons/16-concurrency) | Advanced |
| 17 | Async Rust with Tokio | [`lessons/17-async-with-tokio`](lessons/17-async-with-tokio) | Advanced |
| 18 | Advanced Traits and Trait Objects | [`lessons/18-advanced-traits`](lessons/18-advanced-traits) | Advanced |
| 19 | Macros | [`lessons/19-macros`](lessons/19-macros) | Advanced |
| 20 | Unsafe, FFI and Performance | [`lessons/20-unsafe-ffi-performance`](lessons/20-unsafe-ffi-performance) | Advanced |
| 21 | Project: A CLI Task Manager | [`lessons/21-project-cli-tool`](lessons/21-project-cli-tool) | Projects |
| 22 | Project: REST API with Axum and PostgreSQL | [`lessons/22-project-rest-api`](lessons/22-project-rest-api) | Projects |
| 23 | Project: Concurrent URL Health Checker | [`lessons/23-project-url-checker`](lessons/23-project-url-checker) | Projects |
| 24 | Project: Async Chat Server | [`lessons/24-project-chat-server`](lessons/24-project-chat-server) | Projects |

Each lesson folder has its own `README.md` with how to run it, a table of every step
from the video, and the exercises.

## Getting started

### 1. Install Rust

Install Rust with **rustup** from <https://rustup.rs> (on Windows, also install the
"Desktop development with C++" tools when the installer asks). Then check:

```bash
rustc --version    # 1.85 or newer
cargo --version
```

An editor with **rust-analyzer** helps a lot: VS Code with the rust-analyzer extension, RustRover, or Zed.

### 2. Get the code

```bash
git clone https://github.com/clintbugsdev/rust-beginner-to-advanced.git
cd rust-beginner-to-advanced
```

Or click **Code → Download ZIP** on GitHub and unzip it.

### 3. Run a lesson

```bash
cd lessons/04-ownership
cargo run                      # the final program from the video
cargo run --example step01     # an earlier step from the video
```

The first build downloads and compiles dependencies, so it takes a little longer. After that it's quick.

## How each lesson folder is organised

```
lessons/05-borrowing-and-slices/
├── README.md        how to run it, every step from the video, exercises
├── Cargo.toml
├── src/main.rs      the final working program:          cargo run
├── examples/        earlier working steps:              cargo run --example step03
└── mistakes/        steps that fail to compile on purpose, with the real error at the bottom
```

**About `mistakes/`.** Reading compiler errors is a big part of learning Rust, so the videos
break the code on purpose. Those versions are kept in `mistakes/`, which Cargo doesn't compile.
To see an error yourself, copy one over `src/main.rs` and run `cargo run`, then put the
original back with `git checkout src/main.rs`.

## The best way to learn with this repo

1. Watch the lesson's video.
2. Make your own project with `cargo new my_lesson` and type the code yourself. Typing
   it, and fixing your own typos, teaches far more than copying.
3. When you're stuck, compare with the lesson folder here, step by step.
4. Do the exercises at the end of each lesson's README.

## Commands for the whole repository

All 24 lessons are one Cargo **workspace**, so dependencies compile once into a shared `target/` folder.

```bash
cargo build --workspace --all-targets   # build every lesson, step and test
cargo test --workspace                  # run every test
cargo run -p ownership                  # run a lesson from the top folder (use the package name)
```

## Extra requirements for some lessons

| Lesson | Needs |
|---|---|
| 20: Unsafe, FFI and Performance | A C compiler: gcc or clang (Linux/macOS), or the MSVC build tools (Windows) |
| 22: REST API with Axum and PostgreSQL | PostgreSQL 14 or newer, or Docker. See that lesson's README |
| 23: Concurrent URL Health Checker | Python 3, for the small local test server used in the video (optional) |
| 24: Async Chat Server | `nc` (netcat) to chat by hand (optional: the included `bot` client works everywhere) |

## Troubleshooting

- **`linker 'cc' not found`** (Linux): install your distribution's build tools, e.g. `sudo apt install build-essential`.
- **`link.exe not found`** (Windows): install the Visual Studio C++ build tools, then open a new terminal.
- **`error: package requires rustc 1.85`**: run `rustup update`.
- **Lesson 22 can't connect to the database**: check that PostgreSQL is running and that `DATABASE_URL` is set in the same terminal.
- **Port already in use** (lessons 22 and 24): stop the earlier copy of the server with Ctrl+C.

## About

Made by Clinton L. Bugtong for the ClynzVen Code Academy YouTube channel.
The video narration uses an AI-generated voice; all code and outputs in the videos are real.

The code is released under the [MIT License](LICENSE): use it freely in your own projects.
