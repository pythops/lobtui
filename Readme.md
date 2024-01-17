<div align="center">
  <img height="100" src="assets/logo.png"/>
  <h2> TUI for <a href="https://lobste.rs"> lobste.rs </a> website </h2>
  <img src="https://github.com/pythops/lobtui/assets/57548585/6bf37e19-8a22-46e3-8d8a-8505c0080f06"/>
</div>

## 🚀 Installation

### 📥 Binary release

You can download the pre-built binaries from the release page [release page](https://github.com/pythops/lobtui/releases)

### 📦 crates.io

You can install `lobtui` from [crates.io](https://crates.io/crates/lobtui)

```shell
cargo install lobtui
```

### ⚒️ Build from source

Run the following command:

```shell
git clone https://github.com/pythops/lobtui
cd lobtui
cargo build --release
```

Then run `strip` to reduce the size of the binary

```shell
strip target/release/lobtui
```

This will produce an executable file at `target/release/lobtui` that you can copy to a directory in your `$PATH`.


## 🪄 Usage

`j` or `Down` : scroll down

`k` or `Up`: scroll up

`n`: next page

`p`: previous page

`r`: reload the current page

`o`: open the  highlighted story in the browser


## ⚖️ License

GPLv3
