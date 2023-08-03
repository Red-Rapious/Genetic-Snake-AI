# Genetic-Snake-AI
An Artificial Intelligence to play the game Snake, using a Genetic Algorithm.

## About
The goal is to build a simple Snake game, and to create an artificial intelligence to play the game, and hopefully get high enough scores.

### Motivation
This project is a sequel to both [MLP-Digits-Recognition](https://github.com/Red-Rapious/MLP-Digits-Recognition) and [Genetic-Birds-Simulator](https://github.com/Red-Rapious/Genetic-Birds-Simulator). I decided to apply the genetic algorithm approach to a situation slightly more complicated than the last time. 

### Technical description
- The [`lib-neural-network`](libs/neural-network/src/lib.rs) library contains an implementation of a FFNN (Feed-Forward Neural Network). Conversely to my last two projects, I had performance in mind and used the [`nalgebra`](https://www.nalgebra.org) crate to optimise the computations of the Neural Network.
- The [`lib-genetic-algorithm`](libs/genetic-algorithm/src/lib.rs) library implements a genetic algorithm, which selects, crossovers, and mutates individuals.
- The [`lib-game`](libs/simulation/src/lib.rs) back-end library holds snakes, apples, and is responsible for game update and evolution.
- The [`lib-game-wasm`](libs/simulation-wasm/src/lib.rs) middle-end library is a WebAssembly wrapper for `lib-game`.
- The [`npm` app](www) contains the front-end elements for the simulation and is responsible of displaying snakes, apples, and to handle user input.

## Running the simulation

The simulation can run in any browser. To try it yourself, you will need:
- `rustc` and `cargo` installed, for the back-end code.
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/), to compile Rust code into WebAssembly
- [`npm`](https://www.npmjs.com/get-npm) for the front-end simulation

In the `Genetic-Snake-AI` root folder, you can make sure that everything is working by running:
```console
$ cargo check && cargo test
```
Then, compile the Rust code to WebAssembly by running:
```console
$ cd libs/game-wasm
$ wasm-pack build --release
```
Finally, launch the front-end simulation by running:
```console
$ cd ../..
$ cd www
$ npm run start
```

If everything goes as planned, your terminal will display:
```console
...
｢wds｣: Project is running at http://localhost:8080/
...
｢wdm｣: Compiled successfully.
```

Enter `http://localhost:8080/` (or any other given address) in your favorite web browser, and the simulation should start.

## License
This work is licensed under the [CC-BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) license.