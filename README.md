# Electrum client for Dogecoin

This repository was forked from
[rust-electrum-client](https://github.com/bitcoindevkit/rust-electrum-client) in
an effort to work with Dogecoin in Rust.

The dogecoin patchset is extremely small, currently all we do is:

- Add this section to the README.
- Rename the package.
- Depend upon [rust-dogecoin](https://github.com/tobin-crypto/rust-dogecoin)
  (dogecoin fork of
  [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin)).

## ----- Original rust-electrum-client README ------

# rust-electrum-client [![Build Status]][travis] [![Latest Version]][crates.io]

[Build Status]: https://api.travis-ci.org/MagicalBitcoin/rust-electrum-client.svg?branch=master
[travis]: https://travis-ci.org/MagicalBitcoin/rust-electrum-client
[Latest Version]: https://img.shields.io/crates/v/electrum-client.svg
[crates.io]: https://crates.io/crates/electrum-client


Bitcoin Electrum client library. Supports plaintext, TLS and Onion servers.
