# WEBGL experiments using Rust and WASM

Playing around with WebGL using Rust and WASM.

## Building from source

The Rust project is set up to use [wasm-pack](https://rustwasm.github.io/wasm-pack/) to build it to wasm. Simply execute `wasm-pack build` in the root directory. This will build the Rust code into WASM in the `pkg` directory.

The web component of the project is set up to use NPM and WebPack. Once the Rust code has been compiled, navigate to the `web` directory and execute `npm install` and `npm run build`.

## See also

This work is heavily inspired by Doug Milford's tutorials, which can be found [here](https://www.youtube.com/watch?v=p7DtoeuDT5Y), [here](https://www.youtube.com/watch?v=kjYCSySObDo&t=526s), and [here](https://www.youtube.com/watch?v=K63uBfs1K7Y).

## Licence

This code is free for you to use under the MIT licence.
