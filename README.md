# Walkclean - automated `cargo clean`

`walkclean` is a tool which walks through directories, finds Cargo projects and runs `cargo clean` if they have a `target` directory.
This is useful if you want to `cargo clean` multiple Cargo projects.

## Compatibility
- ✅ Windows
- ✅ macOS
- ✅ Linux

## Usage
Lets assume our current working directory is `rust_projects`. This directory has the following stricture:
```
.
├── another_cool_project
│   ├── Cargo.toml
│   ├── target
│       └── ...
│   ├── Cargo.lock
│   └── src
├── hello_world
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src
└── some_cool_project
    ├── Cargo.toml
    └── src
```

> ⚠️ Note that it does not matter whether the Cargo project is an executable or a library, `walkclean` works with both.

Now, simply run `walkclean` in your terminal:
```sh
$ walkclean
```

`walkclean` will assume that a directory is a Cargo project directory if the following 2 requirements are met:
- A directory named `target` exists
- A file named `Cargo.toml` exists

> ⚠️ The working directory (in this example `rust_projects`) is always ignored, only it's subdirectories are scanned.

In this example, only `another_cool_project` is actually `cargo clean`d. `hello_world` and `some_cool_project` don't have a `target` directory, so they are ignored.

## ~~Command-line arguments~~
Currently `walkclean` does not have any command-line arguments, so you cannot change it's behavior.