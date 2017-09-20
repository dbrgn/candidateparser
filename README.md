# candidateparser

This is a parser for the [ICE](https://tools.ietf.org/html/rfc5245) Candidate
SDP, which is used for connectivity establishment and NAT traversal in
communication systems like WebRTC.

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


## FFI

This library includes C bindings, so you can use it from any language that
supports the C calling convention. An example program can be found in
`candidateparser-ffi/example.c`.


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
