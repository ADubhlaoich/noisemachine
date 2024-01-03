# rustbook

This repository is for my workings through [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) book, commonly known as "The Rust book".

*Conventions*
- `rustup` is used for version management
- Indentation is handled with four spaces
- Cargo is used for package management and building systems
- Variables and references are immutable by default

*Symbols*
- Most Rust lines end with a semicolon `;`
- Rust functions are invoked with `functionName()`
- Function bodies are wrapped in curly braces `{}`
- Rust macros have an exclamation mark after their name `!`
- Associations for libraries, functions and type are marked with a double colon `::`
- Access a specific piece of code from reference `&`

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

*Creating a variable*
```rust
let variableName = value;
```

*Creating a mutable variable*
```rust
let mut variableName = value;
```

*Using part of the standard library*
```rust
use std::io;
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

*Adding crates (libraries) with cargo*  
Append the crate to the `[dependencies]` section, which follows semantic versioning.

*Update project dependencies*
```shell
cargo update
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
