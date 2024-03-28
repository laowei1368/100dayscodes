# Signal Handler in RUST

## As the begin

There is also a practice about RUST workspace. Here is a basic step to manage multiple
binary in one project.
1. Create to folder as the root folder of the project
2. create Cargo.toml, and set workspace section
```rust
[workspace]
```
3. Create first project with cargo
```shell
cargo init signal-with-variable
```
5. Create second project with cargo
```shell
cargo init signal-with-channel
```
6. Modify main.rs for each binary like this
signal-with-variable/src/main.rs
```rust
fn main() {
    println!("Hello, Variables!");
}
```
signal-with-channel/src/main.rs
```rust
fn main() {
    println!("Hello, Channels!");
}
```
7. build each project individually
```shell
cargo build -p signal-with-variable
cargo build -p signal-with-channel
```
or build them togeter
```shell
cargo build
```
8. run each binary
```shell
cargo run -p signal-with-variable
cargo run -p signal-with-channel
```