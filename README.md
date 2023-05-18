# First setup
Create .env based on the .env_template provided in the repo
cargo install cargo-watch
cargo install sqlx-cli
export PATH=$PATH:/home/thth/.cargo/bin
sqlx migrate run
# Launch in dev mode
RUST_BACKTRACE=1 cargo watch -q -c -w src/ -x run

# Cargo shenanigans
## Cargo release compilation command:
```
cargo +nightly build -Z build-std=std,panic_abort --target x86_64-unkown-linux-gnu --release
```
## cargo-clippy for idiomatic and recommandations (basically a somewhat "advanced" linter)
```
cargo clippy
```
## cargo-fmt for formatting
```
cargo fmt
```
## cargo-watch for dev reloads
```
cargo install cargo-watch
cargo watch -q -c -w src/ -x run
```
## sqlx-cli tool for migrations
```
cargo install sqlx-cli
export PATH=$PATH:/home/thth/.cargo/bin
```
sqlx-cli example commands:
```
sqlx database create openpost-db
sqlx migrate run
```
# Useful tools
## unused-features
__unused-features__ for optimising import (slows with multiple compiles). Do not cancel it midrun,
it modifies Cargo.toml during analysis and that will mess with imports.
[crate repo:](https://github.com/TimonPost/cargo-unused-features)

```
cargo install cargo-unused-features
//Analysis 
unused-features analyze
//Applying report results
unused-features prune --input ./report.json
```
## cargo-bloat
Calculate various things size in your final binaries. Requires .symbol to be enable.
You must not have strip = false in your compilation profile for it to work.
```
cargo install cargo-bloat
// Size of imports
cargo bloat  --release --crates
// Size of 10 biggest things
cargo bloat --release -n 10
[crate repo:](//https://github.com/RazrFalcon/cargo-bloat)

# References
[Profile optimisation for compilation](https://github.com/johnthagen/min-sized-rust)
[Cross compilation for arm and risc-v](https://kerkour.com/rust-cross-compilation
[cross-rs wiki](https://github.com/cross-rs/cross/wiki/Getting-Started)
Note: it is possible to make Cross.toml files to specify custom docker images and compilation flags

# RoadMap
TODOs:
- Fix Error handling and refactor handlers.rs
- Refactor models for proper impl per structure for their key methods
- Implement tls: axum-server supports directly encrypted-stream provided for https support.
[tls example from axum](https://github.com/tokio-rs/axum/tree/main/examples/tls-rustls)
- Specify logging requirements:
[tracer logging example](https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/src/main.rs)
- Ungarbagify SQL tables/Schema/Models :)
[What not to keep?](https://dev.mysql.com/blog-archive/storing-uuid-values-in-mysql-tables/)
- Explore cargo test and cargo bench for tests and benchmark
[test wiki](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
[test guidelines](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html)
