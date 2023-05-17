# First setup
Create .env based on the .env_template provided in the repo
cargo install cargo-watch
cargo install sqlx-cli
export PATH=$PATH:/home/thth/.cargo/bin
sqlx migrate run
# Launch in dev mode
RUST_BACKTRACE=1 cargo watch -q -c -w src/ -x run