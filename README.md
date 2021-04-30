# A simple ERC20 contract built with ink! for substrate and polkadot

## Prerequisite

### Install `Substrate` env
[Follow this steps](https://substrate.dev/docs/en/knowledgebase/getting-started/) to setup substrate development environment

### Install ink! command line (CLI) utility

Need to install the `binaryen` package, which is used to optimize the WebAssembly bytecode of the contract.

Many package managers have it available ‒ e.g. there is a package for [Debian/Ubuntu](https://tracker.debian.org/pkg/binaryen), [Homebrew](https://formulae.brew.sh/formula/binaryen) and [Arch Linux](https://archlinux.org/packages/community/x86_64/binaryen/). If there is no proper package you can download the source code from github [binaryen](https://github.com/WebAssembly/binaryen) and compile for the binary.

After you've installed the package execute:

```
cargo install cargo-contract --vers ^0.12 --force --locked
```

We can check the installation using `cargo contract --help`

### Test the smart contracts

The smart contract code is available in the 'lib.js' file. To test it you can execute the below commands.

Quickly check that it compiles and the trivial test passes with:
```
cargo +nightly test
```

Also check that you can build the Wasm file by running:
```
cargo +nightly contract build
```

If everything looks good, then we are ready with the smart contract!
