libconsensus
===========
[![Build Status](https://travis-ci.org/Fantom-foundation/libconsensus.svg?branch=master)](https://travis-ci.org/Fantom-foundation/libconsensus)

libconsensus in Rust.

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Description

This crate defines a set of commonly used traits which can be used for various consensus
implementations. The crate defines two traits: ConsensusConfiguration and Consensus. The crate
also defines a base struct (BaseConsensusPeer) which can be used between multiple consensus algorithms.

For an example of an implementation of the traits, refer to the libconsensus-dag repository:
https://github.com/Fantom-foundation/libconsensus-dag.

### Step-by-step guide
```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone https://github.com/Fantom-foundation/libconsensus && cd libconsensus
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```

### Example

#### Prelude
```rust
use libconsensus::ConsensusType;
use libconsensus_dag::{DAGconfig}
use libhash_sha3::Hash as EventHash;
use libsignature_ed25519_dalek::{SecretKey, PublicKey, Signature};
```

**Prepare consensus configuration**
```rust
type Id = libsignature_ed25519_dalek::PublicKey;
pub struct Data {
  int d,
};
let consensus_type = config.read().unwrap().consensus_type.clone();

let consensus_config = {
  match consensus_type {
      libconsensus::ConsensusType::DAG => {
        cfg = DAGconfig::<Id, Data, SecretKey, PublicKey>::new();
        cfg.transport_type = libtransport::TransportType::TCP;
        cfg.store_type = libconsensus_dag::store::StoreType::Sled;
        let (public_key, secret_key) = Signature::<EventHash>::generate_key_pair()?;
        cfg.creator = public_key;
        cfg.secret_key = secret_key;
        let mut peers = libconsensus_dag::DAGPeerList::<Id, PublicKey>::new();
        peers.get_peers_from_file('peers.json')?;
        cfg.peers = peers;
        cfg
      }
      libconsensus::ConsensusType::Unknown => panic!("unknown consensus type"),
  }
}
```
