# simple-admin a backend system for managing users and roles use rust
> this is my first open source project, I will use rust and rust wasm build this project.
> Let's work together to improve this project.
UI Reference: https://github.com/robbins23/daisyui-admin-dashboard-template
## How it looks
![login](https://raw.githubusercontent.com/gwbiubiu/iamge_public/refs/heads/main/simple_admin/login.gif "login")
## start server
```bash
cargo run -p server
```
## start web
first install daisyui dependency
```bash
npm install
```
second you need to install wasm environment
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli
```
then you can start web
```bash
trunk serve
```