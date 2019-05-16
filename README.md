Built on top of boilerplate from https://github.com/rustwasm/rust-webpack-template

## Motivation
The aim of the project is providing a performant XSS-sanitizer for the client side. This is done implementing the library in Rust and using WebAssembly as the compile target.

A client side XSS-sanitizer could be needed e.g. in situations the client is utilizing an untrusted API, or is connected to other users through a P2P network.

The project was made as a part of a university course on information security.

## Usage
In this demo we compile both the .wasm binary, and the .js source via webpack. Build and start the demo by running npm start in project root.
