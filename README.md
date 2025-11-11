# Hashmoney

Yet another implementation of [hashcash](hashcash.org).

You can either mint a stamp or parse it from a string, then check if it is valid against some given parameters.
Only version 1 of the specification is implemented.
The `wasm32-unknown-unknown` target is a first class citizen and fully supported with the `wasm` feature.

### Usage

```toml
[dependencies]
hashmoney = { version = "2.0.0" }
```
