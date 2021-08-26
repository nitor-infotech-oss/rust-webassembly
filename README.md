<div align="center">

  <h1><code>wasm-calculator</code></h1>

  <strong>Web Assembly Calculator POC <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  

 

</div>

## About

This poc is designed for creating a Calculator app in Web Assembly and Rust

## Requirements
Befor start with this poc You need to install Rust, Node

* First, complete the basic [Rust setup instructions](https://www.rust-lang.org/tools/install).
* Second, complete the Node installation [Install Node](https://nodejs.org/en/download/).

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name wasm-calculator
cd wasm-calculator
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
  


Directory Structure
-------------------

      src/       under this dir we have lib.rs file where we write all functions required for calculator
      test/      under this dir we write all test cases for all calculator operations
      www/       under this folder we write javascript code (UI part of wasm-calculator)
