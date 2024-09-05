<div align="center">
  <img height="100" src="assets/logo.png"/>
  <h2> TUI for <a href="https://lobste.rs"> lobste.rs </a> website </h2>
  <img src="https://github.com/user-attachments/assets/13584eba-7373-4516-9f42-1b16651cd9e6"/>
</div>

## 🚀 Installation

### 📥 Binary release

You can download the pre-built binaries from the release page [release page](https://github.com/pythops/lobtui/releases)

### 📦 crates.io

You can install `lobtui` from [crates.io](https://crates.io/crates/lobtui)

```shell
cargo install lobtui
```

## 🐧 Arch Linux

You can install [`lobtui`](https://aur.archlinux.org/packages/lobtui) from the AUR using your favorite [AUR helper](https://wiki.archlinux.org/title/AUR_helpers). For example:

```shell
paru -S lobtui
```

## 🚩 NetBSD

You can install [`lobtui`](https://https://pkgsrc.se/editors/lobtui) from the official repositories.

```shell
pkgin install lobtui
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

`j` or `Down` : scroll down.

`k` or `Up`: scroll up.

`gg`: go to the top of the page.

`G`: go the bottom of the page.

`n`: next page.

`p`: previous page.

`r`: reload the current page.

`o`: open the highlighted story in the browser.

## ⚖️ License

GPLv3
