Built on top of boilerplate from https://github.com/rustwasm/rust-webpack-template

## Motivation
The aim of the project is providing a performant XSS-sanitizer for the client side. This is done implementing the library in Rust and using WebAssembly as the compile target.

A client side XSS-sanitizer could be needed e.g. in situations where the client is utilizing an untrusted API, or is connected to other clients via a P2P network.

The project was made as a part of a university course on information security.

## Project Description
This is a demo repository for the actual libary which has not been published on its own at this time. The library source can be found in /crate/src. 

The library has been implemented following the [OWASP XSS Prevention Cheat Sheet](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Cross_Site_Scripting_Prevention_Cheat_Sheet.md). It contains encoding implementations for rules 1-5, and can be utilized to enforce rule 8. The rest of the rules are outside of the scope of this library.

#### The Code
The file lib.rs contains the API exposed by the library, the encoders -folder contains the encoder implementations the library provides.

#### Performance
The html_entity_encoder has been optimized by handling the input as 8-bit signed integers (as in, utf-8 character codes) in a vector. This, although increasing the runtime memory footprint, yielded somewhere around 30-60% performance increase over handling the input as characters or using library string replace methods on a mutable input. Note that the performance tests used are extremely crude, they can be found in the file js/index.js. The library performs around 6-7x faster than a naive javascript implementation. The library is yet to be compared with an actual open source client-side encoding library.

Other methods I've tried were utilizing regexes and a library implementing the [Aho-Corasick string-searching algorithm](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm). These were not nearly as performant as the current solution.

Other encoders (that is, apart from html_entity_encoder) miss some optimization steps at the moment.

Another optimization target the library still suffers from is strings that do not have to be encoded. The performance here could probably be improved by doing a regex-search on the input string before starting the encoding process.

#### Testing
At the moment the library has mainly been tested by hand, utilizing various xss attack vector collections:
https://gist.github.com/soaj1664/9588791  
https://gist.github.com/JohannesHoppe/5612274  
https://www.owasp.org/index.php/XSS_Filter_Evasion_Cheat_Sheet

Most of the testing has been done with html entities, but the other encoders have also been tried out.

The library includes very rudamentary unit tests, found in crate/src/test. These can be ran via the command `cargo test` in the library src folder. More unit testing is necessary for better code coverage.

## The Demo
The demo is not ran on a public server, so it has to be set up locally.

## Prerequisities
Rust 1.30.0 or later, npm. Probably other stuff I'm forgetting, like openssl and pkg-config.

## Configuration
In this demo we compile both the .wasm binary, and the .js source via webpack. Build and start the demo by running `npm install` and `npm start` in the project root. This will install dependencies, compile and package the rust source code to a WebAssembly module via [wasm-pack](https://github.com/rustwasm/wasm-pack), and build the javascript bundles served by the webpack test server that start listening on a local port (most likely localhost:8080).

The demo contains two input fields that propagate their contents to a <div/> on blur. One of them is sanitized with the .wasm module, one is not. 

There's also a file upload handle for text files which are ran through a naive js implementation of html encoding and the .wasm module respectively. The time taken to complete the operation in ms is logged to the browser console.
