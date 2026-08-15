cargo --version
cargo new rust-standard

cargo run --bin myapp
cargo run --example demo

cargo check
cargo build
cargo build --release
cargo build --release --bin template_main
cargo build --release --all-targets

install libary
cargo add tracing@0.1.44
cargo add tracing-subscriber@0.3.23

target\release\rust-standard.exe
target\release\template_main.exe