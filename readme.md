# Solare

Tooling for interacting with [solana](https://solana.com/) blockchain with examples of solana programs.


## Setup

* These tools require docker installed into your system
* Build the solana image with `make image-solana`
* Enter the image with `make solana`
* Interact with `solana` cli
  ```
  solana --version
  solana config get
  solana cluster-version
  ```

## [Solana Clusters](https://docs.solana.com/clusters)

To connect to devnet
```
solana config set --url https://api.devnet.solana.com
```

## Wallets

To play around with the image you can start by creating [file system wallets](https://docs.solana.com/wallet-guide/file-system-wallet) (bear in mind that these are the most unsecure wallets and should be used only when interacting with the `devnet`).

The image mount the `$HOME/.solana-wallets` directory so that you can create file wallets persisting on your hard disk:

```
solana-keygen new --outfile ~/.solana-wallets/my-keypair.json
```

To show the public key
```
solana-keygen pubkey ~/.solana-wallets/my-keypair.json
```

## Update Image

Use the latest [release](https://github.com/solana-labs/solana/releases/latest) tag when updating the Dockerfile.

## Useful Repos

* [solana-labs](https://github.com/solana-labs) - Solana foundation repos
* [project-serum](https://github.com/project-serum) - Serum DEX and anchor framework for aolana programs
* [ironaddicteddog](https://github.com/ironaddicteddog) - few anchor solana program implementation
