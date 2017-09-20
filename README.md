# ice-candidate-parser

This is a parser for the ICE Candidate SDP, for example:

    candidate:842163049 1 udp 1686052607 1.2.3.4 46154 typ srflx raddr 10.0.0.17 rport 46154 generation 0 ufrag EEtu network-id 3 network-cost 10

It will parse the data into a struct with all components.

For more information, see [RFC5245 (Interactive Connectivity Establishment (ICE))](https://tools.ietf.org/html/rfc5245).
