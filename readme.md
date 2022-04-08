# Solare

[![build](https://github.com/quantmind/solare/actions/workflows/build.yml/badge.svg)](https://github.com/quantmind/solare/actions/workflows/build.yml)

Tooling for interacting with [solana](https://solana.com/) blockchain with examples of solana programs.


## Setup

* These tools require docker installed into your system
* Build the solare image with `make image-solare`
* Run the solana test validator `make solana-test-validator`
* Enter the image with `make solare`
* Interact with `solana` cli
  ```
  solana --version
  solana config get
  solana cluster-version
  ```

## [Solana Clusters](https://docs.solana.com/clusters)

Enter the image via `make solare`

* `solana-local` to connect to the running local `solana-test-validator`
* `solana-dev` to connect to devnet
* `solana-prod` to connect to mainnet

## Wallets

To play around with the image you can start by creating [file system wallets](https://docs.solana.com/wallet-guide/file-system-wallet) (bear in mind that these are the most unsecure wallets and should be used only when interacting with the `devnet`).

```
solana-keygen new
```

To show the public key
```
solana-keygen pubkey
```

## Update Image

Use the latest [release](https://github.com/solana-labs/solana/releases/latest) tag when updating the Dockerfile.

## Useful Repos

* [solana-labs](https://github.com/solana-labs) - Solana foundation repos
* [project-serum](https://github.com/project-serum) - Serum DEX and anchor framework for aolana programs
* [ironaddicteddog](https://github.com/ironaddicteddog) - few anchor solana program implementation
