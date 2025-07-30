# Hashmoney

Yet another implementation of [hashcash](hashcash.org).
It has a way simple interface, so that the resulting workflow is more like using the command line version.
You can either mint a stamp or parse it from a string, then check if it is valid against some given values.
Only version 1 of the specification is implemented.
The Web assembly target is a first class citizen and fully supported with the `wasm` feature.

### Usage

```toml
[dependencies]
hashmoney = { version = "2.0.0" }
```
