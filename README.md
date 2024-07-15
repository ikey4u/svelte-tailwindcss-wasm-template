# svelte-tailwindcss-wasm-template

This repository contains starter template for svlete, tailwindcss and wasm
combinations template. The wasm is powered by rust.

## Quick Start

Once you have installed `rust` and `npm`, run following command to install
dependencies:

    npm i
    cargo install -f wasm-bindgen-cli

Then start to development use command:

    npm run dev

To build the project, run:

    npm run build

The built website can be found under `build` directory.

## Project Structure

    /
    +-- /src
        +-- lib/
            +-- wasm/: wasm library source
        +-- routes/: website pages
        +-- app.css: application style
        +-- app.html: application entry
