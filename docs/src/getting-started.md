# Getting Started

## Prerequisites
- Rust stable toolchain and `cargo`
- mdBook for local docs

Install mdBook:
```bash
cargo install mdbook
```

## Local docs preview
Run a live-reloading dev server:
```bash
mdbook serve docs -p 3000 -n 127.0.0.1
```
Open http://127.0.0.1:3000

## Build full site locally (docs + Rust API)
```bash
# Build mdBook into ./site
mdbook build docs --dest-dir site

# Build Rust API docs
cargo doc --workspace --all-features --no-deps

# Assemble into the site
mkdir -p site/api/rust
cp -r target/doc/* site/api/rust/
# Prevent Jekyll processing on GitHub Pages
touch site/.nojekyll
```
Open `site/index.html` in your browser.

## CI/CD
A GitHub Actions workflow publishes the site to GitHub Pages on pushes to `main`.
