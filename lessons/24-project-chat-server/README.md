# Lesson 24: Project: Async Chat Server

*Part 4 · Projects* · Video 24 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

A multi-user TCP chat server with Tokio: one task per client, a broadcast channel, shared state, and select!.

## Run it

Start the server with `cargo run` (it listens on 127.0.0.1:7878). In other terminals, join with `nc 127.0.0.1 7878` (type your name first, then messages; `/who` lists who's online), or run the scripted test client: `cargo run --bin bot -- ana 0 "hello everyone|/who"`.
