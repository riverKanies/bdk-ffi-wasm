## Idea
the idea for this repo is to make a bdk-wasm lib that mirrors the api of bdk-ffi

## Notes:
translating rust from [bdk-ffi](https://github.com/bitcoindevkit/bdk-ffi/tree/master/bdk-ffi) to a wasm api, steps:

- copy struct and impl from bdk-ffi/bdk-ffi/src files
- make wrapper struct for struct i.e. Wallet → WalletWrapper
- add wallet attr to wrapper, use Rc<RefCell<Wallet>> pattern
- add wasm_bindgen trait to wrapper and wrapper methods that get exposed to JS
- this will raise errors for all methods that aren’t wasm compatible i.e. have string or number I/O
- ensure wrapper constructor returns wrapper with Rc::new and RefCell::new on the inner struct
- wrappers are wasm compatible and can be used as proxy objects in JS for underlying rust struct
- wrapper methods must pull self inner out of wrappers and clone with Rc::clone
- wrapper methods must also wrap return values that aren’t numbers or strings
- wrappers that get passed in as values to methods for other wrappers must have a 'get' method to access and clone the inner value from another wrapper struct. If the inner struct doesn't have a Clone impl, you must make a 'newtype' struct to implement clone, and From impl to convert back to the og BdkType (see Types.rs>FullScanRequest)









<br/><br/><br/><br/><br/><br/><br/><br/><br/><br/>


# OG Wasm Pack README:

<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
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
