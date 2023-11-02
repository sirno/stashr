# STASHR

This is stashr.

With stashr you can stash a file wherever you are.

And then get it back wherever you will be.

## Usage

Stash some files

```bash
stashr foo.txt bar.txt
```

Get them back

```bash
stashr
```

## Install

```bash
cargo install stashr
```

### Troubleshooting

You need to be on the nightly release channel of rust to install stashr.
If you are not, you might get the following error: ``error[E0554]: `#![feature]` may not be used on the stable release channel``.

To proceed first install [rustup](https://rustup.rs/), then run the following two commands:

```bash
rustup install nightly
cargo +nightly install stashr
```
