# candidateparser

[![CircleCI](https://circleci.com/gh/dbrgn/candidateparser/tree/master.svg?style=shield)](https://circleci.com/gh/dbrgn/candidateparser/tree/master)
[![Crates.io Version][crates-io-badge]][crates-io]
[![Crates.io Downloads][crates-io-download-badge]][crates-io-download]
[![Rust][rust-badge]][github]

This is a parser for the [ICE](https://tools.ietf.org/html/rfc5245) Candidate
SDP, which is used for connectivity establishment and NAT traversal in
communication systems like [WebRTC](https://webrtc.org/).

Example candidate SDP:

    candidate:842163049 1 udp 1686052607 1.2.3.4 46154 typ srflx raddr 10.0.0.17 rport 46154 generation 0 ufrag EEtu network-id 3 network-cost 10

It will parse the data into a struct with all components. The example above
will result in the following object:

    IceCandidate {
        foundation: "842163049",
        component_id: 1,
        transport: Udp,
        priority: 1686052607,
        connection_address: V4(1.2.3.4),
        port: 46154,
        candidate_type: Srflx,
        rel_addr: Some(V4(10.0.0.17)),
        rel_port: Some(46154),
        extensions: Some({
            [110, 101, 116, 119, 111, 114, 107, 45, 99, 111, 115, 116]: [49, 48],
            [103, 101, 110, 101, 114, 97, 116, 105, 111, 110]: [48],
            [117, 102, 114, 97, 103]: [69, 69, 116, 117],
            [110, 101, 116, 119, 111, 114, 107, 45, 105, 100]: [51]
        })
    }

For more information, see [RFC5245 (Interactive Connectivity Establishment (ICE))](https://tools.ietf.org/html/rfc5245).

Crate docs: https://docs.rs/candidateparser/

Note: Due to build system issues, Rust 1.21+ is required (currently in Beta).


## FFI

This library includes C bindings, so you can use it from any language that
supports the C calling convention. An example program can be found in
`candidateparser-ffi/example.c`.

### Building for Android

Requirements:

- Android NDK
- Rustup
- Make

Add rustup targets:

    $ rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android

Build standalone toolchains:

    $ ./create-ndk-standalone.sh

Add toolchain config to your `~/.cargo/config`:

    $ cat cargo-config.toml >> ~/.cargo/config

Build Android libraries:

    $ make android


## Linting

To run clippy lints, compile the library with `--features clippy` on a nightly compiler:

    $ cargo build --features clippy

Note: Running this command from the workspace root won't work, you have to `cd`
into the sub-crates!


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.


### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


[crates-io]: https://crates.io/crates/candidateparser
[crates-io-badge]: https://img.shields.io/crates/v/candidateparser.svg?maxAge=3600
[crates-io-download]: https://crates.io/crates/candidateparser
[crates-io-download-badge]: https://img.shields.io/crates/d/candidateparser.svg?maxAge=3600
[github]: https://github.com/dbrgn/candidateparser
[rust-badge]: https://img.shields.io/badge/rust-1.21%2B-blue.svg?maxAge=3600
