Built on top of boilerplate from https://github.com/rustwasm/rust-webpack-template

## Motivation
The aim of the project is providing a performant XSS-sanitizer for the client side. This is done implementing the library in Rust and using WebAssembly as the compile target.

A client side XSS-sanitizer could be needed e.g. in situations where the client is utilizing an untrusted API, or is connected to other clients via a P2P network.

The project was made as a part of a university course on information security.

## Prerequisities
Rust 1.30.0 or later, npm, probably other stuff like openssl

## Running the Demo
In this demo we compile both the .wasm binary, and the .js source via webpack. Build and start the demo by running `npm install` and `npm start` in the project root.

The demo contains two input fields that propagate their contents to a <div/> on blur. One of them is sanitized with the .wasm module, one is not. 

There's also a file upload handle for text files which are ran through a naive js implementation of html encoding and the .wasm module respectively. The time taken to complete the operation in ms is logged to the browser console.
