# rustbook

This repository is for my workings through [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) book, commonly known as "The Rust book".

*Conventions*
- `rustup` is used for version management
- Indentation is handled with four spaces
- Cargo is used for package management and building systems


*Symbols*
- Most Rust lines end with a semicolon `;`
- Rust functions are invoked with `functionName()`
- Function bodies are wrapped in curly braces `{}`
- Rust macros have an exclamation mark after their name `!`

*Updating Rust*
```shell
rustup update
```

*Uninstalling Rust*
```shell
rustup self uninstall
```


*Compiling a single file*
```shell
rustc <File.rs>
```

*A basic function*
```rust
fn <name>() {

}
```

*A basic Rust macro*
```rust
println!("Hello, world!");
```

*Creating a new project*
```shell
# Creates a project directory
cargo new <project-name>
```

*Validate a project will compile*
```shell
cargo check
```

*Running a project*
```shell
cargo run
```

*Building a project*
```shell
cargo build
```

*Build a project with release optimisations*
```shell
cargo build --release
```



