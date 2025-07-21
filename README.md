# Hashmoney

> [!WARNING]
> Work in progress

This is yet another implementation of [hashcash](hashcash.org), but also very different.
It has a way simple interface, so that the resulting workflow is more like using the command line version.
You can either mint a stamp, or check if it is valid.
Only version 1 of the specification is implemented.
The Web assembly target is a first class citizen and fully supported with the `wasm` feature.

### Usage

```toml
[dependencies]
hashmoney = { version = "1.0.0" }
```
