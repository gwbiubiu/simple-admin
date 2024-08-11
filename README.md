# simple-admin a backend system for managing users and roles use rust
> this is my first open source project, I will use rust and rust wasm build this project.
> Let's work together to improve this project.
## start server
```bash
cargo run -p server
```
## start web
first you need to install wasm environment
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
```
then you can start web
```bash
trunk serve
```