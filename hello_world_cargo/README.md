# Hello world using CARGO

1. Cargo is Rust’s build system and package manager.
2. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

Having few files works fine for the small projects , but in the real world projects where they will be a lot of files with dependencies
where we need something to handle that.

Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

If you started a project that doesn’t use Cargo, as we did with the “Hello, world!” project, you can convert it to a project that does use Cargo. Move the project code into the src directory and create an appropriate Cargo.toml file.

## RUST Comes with PACKAGE MANAGER CALLED CARGO

1. To create a new package for rust with cargo

```shell
cargo new <file-name>
```

2. If there are some files exists in that folder use this

```bash
cargo init
```

- cargo.toml => which is just like package.json

### To compile a file with cargo

1. Build and run the program using cargo

```shell
cargo build
cargo run // can directly use this , it will build and run the program at the same time
```

2. We can check for the errors in the program without producing the executable file, whenever we want to check if our program is compliing properly
   this process will be fast because it doesnt produce the executable file

```shell
cargo check
```
