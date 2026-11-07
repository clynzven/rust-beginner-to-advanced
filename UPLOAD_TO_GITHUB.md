# How to put this repository on GitHub

You only need to do this once. (This file is just for you; delete it before pushing if you like.)

## Option A: with the GitHub website and Git

1. Go to <https://github.com/new>.
   - Repository name: `rust-beginner-to-advanced`
   - Public
   - Do **not** tick "Add a README", ".gitignore" or "license" (this folder already has them).
   - Click **Create repository**.
2. In a terminal, inside this folder:

```bash
git init -b main
git add .
git commit -m "Rust: Beginner to Advanced, all 24 lessons"
git remote add origin https://github.com/clintbugsdev/rust-beginner-to-advanced.git
git push -u origin main
```

When Git asks for a password, use a **personal access token** (GitHub → Settings →
Developer settings → Personal access tokens), not your account password.

## Option B: with the GitHub CLI

```bash
gh auth login
git init -b main && git add . && git commit -m "Rust: Beginner to Advanced, all 24 lessons"
gh repo create rust-beginner-to-advanced --public --source . --push
```

## After pushing

- Open the **Actions** tab: the CI workflow builds and tests all 24 lessons on Linux, macOS and Windows.
  The badge at the top of the README turns green when it passes.
- On the repository page, click the gear next to **About**: add a description
  ("Source code for the 24-lesson Rust: Beginner to Advanced video course") and the topics
  `rust`, `rust-lang`, `learn-rust`, `tutorial`, `course`.
- Add the repository link to every video description, in place of `[LINK]`.
- If your GitHub username or repository name is different, change the two links at the top of `README.md`.
