# WAtherSM

WebAssembly weather app

## Demo
[Click here](https://yaxincheng.github.io/WAtherSM/)

## Weather API
This demo is based on [OpenWeather](https://openweathermap.org)

##Compile
- request api key from OpenWeather
- put the api key as a file named `.apikey` at the same level of the `Cargo.toml`
- clone this repo
    - `git clone https://github.com/YaxinCheng/WAtherSM`
- install rust
    - More info can be found on [rust-lang.org](https://www.rust-lang.org/tools/install)
- compile with stable version of rust
    - `wasm-pack build --target web --out-name weather_wasm --out-dir ./static`
    - or simply call `./build.sh`
- serve with any http server software
    - `serve.sh` uses [node.js http-server](https://www.npmjs.com/package/http-server)

## Technology
- [Rust](https://www.rust-lang.org)
- [Yew](https://yew.rs)

## Resources
All animations and icons are from internet. I will take them down if it infringes your copyright.

All resources can be found on [gh-pages branch](https://github.com/YaxinCheng/WAtherSM/tree/gh-pages)
