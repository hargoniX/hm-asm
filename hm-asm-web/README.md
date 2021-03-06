# hm-asm-web

This is a WebAssembly based frontend for `hm-asm-simulator`. It expoes
two functions to javascript.
- `simulate(code: &str, cycles: usize)`, its return value is equivalent to the one of `hm_asm_simulator::simulate::simulate`
- `assemble(code: &str)`, its return vlaue is equivalent to the one of `hm_asm_simulator::generate::generate_binary`

If either of the functions should fail they will return their Rust error as a String. In the case of a grammar based
error, this would be a nice pest.rs error.

## Building it

You can build this crate by installing `wasm-pack` and running
```
$ wasm-pack build
```

## Demo
In `demo/` You'll find a demo app that compiles and simulates a simple program inside of the console.
You can test it with

- `npm install`
- `npm run start`

Afterwards you can visit the demo at `localhost:8080`.

## Using it

You can use this guide https://rustwasm.github.io/docs/book/game-of-life/setup.html
to set up your own project. I'm not planning to publish to npm.org at this point.
