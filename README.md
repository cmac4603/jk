# jk
A very opinionated CLI workflow tool.

Available on linux w/gnu & macos.

## Initial Install

### Using rust
- Ensure you have rust/cargo installed with [rustup](https://www.rust-lang.org/learn/get-started):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Cargo binary directory needs to be in PATH:
  ```bash
  export PATH=$HOME/.cargo/bin:$PATH
  ```
- Compile the binary from source:
  ```bash
  cargo install --git ssh://git@github.com/cmac4603/jk jk
  ```

### Download binary from GitHub
- Go to the [releases page](https://github.com/cmac4603/jk/releases).
- Download the binary for your architecture and place in directory under `$PATH`.

## Updating
```bash
jk update
```
