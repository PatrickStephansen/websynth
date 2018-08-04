# Websynth


## Rust to WASM toolchain

* Install `rustup` with: `curl https://sh.rustup.rs -sSf | sh`. Use the default settings.
* Follow the instructions at https://rustwasm.github.io/book/game-of-life/setup.html. Skip `wasm-opt` for now.
* Install wasm-bindgen `cargo +nightly install wasm-bindgen-cli`

## Build

* `npm run build`
* TODO: Minimise build size with `wasm-gc target/wasm32-unknown-unknown/release/websynth.wasm -o target/wasm32-unknown-unknown/release/websynth.gc.wasm`

## Hosting

A web host that supports the 'application/wasm' MIME type is needed for proper wasm streaming. Install `https` using `cargo install https`. OpenSSL was missing on my machine, so I had to run `sudo apt-get install libssl-dev` before that would work. Once installed, run the site with the command `http` from the root of this repo.

Easier: `npm start` to run webpack dev server

## Resources

* Web Audio API standard: https://webaudio.github.io/web-audio-api/
* Web assembly audio worklet example: https://github.com/GoogleChromeLabs/web-audio-samples/tree/gh-pages/audio-worklet/design-pattern/awn
* Rust WASM tutorial: https://rustwasm.github.io/book/game-of-life/

## Architecture

Don't over-think it. Prototype first, design something coherent later. Don't use frameworks for the POCs.

Big issues:

1. Generate audio data in Rust and output sound from the browser. 
1. Take parameters from the user for audio generation.
1. Stream visualisations of the audio to the front-end.

### Project values

1. Maintainablility - this will never get anywhere if it isn't fun to work on
1. Ease of use - this should be a friendly introduction to software synthesis
1. Correctness - do it right or not at all
1. Robustness/Reliability - needs to work consistenly enough for live use. should fail in the least embarrassing ways possible
1. Efficiency - performance should be good enough to perform live music with this synth
1. Portability - portability across operating systems should come automatically. Portability across browsers doesn't matter at all
1. Security - there's nothing of value to protect. Storing anything is a stretch goal.

### Audio

Do as much audio processing as possible in Rust/WASM. Input may need to come from js depending on how the web midi standard is implemented (research needed). Output should be possible using an audio worklet implemented in WASM. The UI will be implemented with frameworkless js with ESM modules (http://exploringjs.com/es6/ch_modules.html). A framework won't be introduced unless one is discovered to perfectly serve the purpose. 

### UI

UI events in the js space will call functions published by the Rust code. Displaying visualisations of audio will be pivotal to the design strategy. Again, as much as possible will be done to render frames on the Rust side. 

## Audio worklets

WASM in combination with worklets is just not viable at present. Loading a WASM module in the context of a worklet is hard, and bindgen can't be used there. Once in the context of a worklet, we lose scope of the DOM, so construting a whole synth in one node is not viable.

The options are, use something in the window scope instead of worklets, or don't use WASM.

For now, I choose the latter, and will start another project in pure js.
