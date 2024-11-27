# WebAssembly demo

## Hướng dẫn chạy

### Cài Rust

https://www.rust-lang.org/tools/install

---

### Cài wasm-pack

```
cargo install wasm-pack
```

---

### Chạy wasm-pack

```
wasm-pack build
```

---

### Chạy Web:

```
npx http-server -p 3000
```

hoặc

```
npx http-server -p 3000 -c-1 --mime "application/wasm:wasm"
```