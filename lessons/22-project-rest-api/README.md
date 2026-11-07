# Lesson 22: Project: REST API with Axum and PostgreSQL

*Part 4 · Projects* · Video 22 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

A notes service with Axum, sqlx and a real PostgreSQL database: routing, JSON, migrations, errors and status codes.

## Run it

Needs PostgreSQL. Set `DATABASE_URL` (see below), then `cargo run`, and try it with `curl` as in the video.

```bash
# Start PostgreSQL with a "notes" database (or use your own server and `createdb notes`):
docker run -d --name notes-db -p 5432:5432 -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=notes postgres:16
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/notes   # Windows PowerShell: $env:DATABASE_URL="..."
cargo run          # runs the migration, then listens on http://127.0.0.1:3000

# In another terminal:
curl -s -X POST localhost:3000/notes -H 'content-type: application/json' -d '{"title":"Hello","body":"My first note"}'
curl -s localhost:3000/notes
```
