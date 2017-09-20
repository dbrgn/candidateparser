//! ICE Candidate parser following RFC5245.
//!
//! The main entry point for this library is the [`parse`](fn.parse.html)
//! function. See the docs of that function for more information.

#[macro_use]
extern crate nom;

pub mod parsers;
mod types;

use nom::IResult::{Done, Error, Incomplete};

pub use types::{IceCandidate, CandidateType, Transport};

/// Parse an SDP bytestring, return an
/// [`IceCandidate`](struct.IceCandidate.html) struct.
///
/// ## Example
///
/// The output for the following ICE candidate:
///
/// ```sdp,ignore
/// candidate:373990095 1 udp 41885439 5.148.189.205 63293 typ relay
/// ```
///
/// ...is the following struct:
///
/// ```rust,ignore
/// IceCandidate {
///     foundation: "373990095",
///     component_id: 1,
///     transport: Udp,
///     priority: 41885439,
///     connection_address: V4(5.148.189.205),
///     port: 63293,
///     candidate_type: Relay,
///     rel_addr: None,
///     rel_port: None,
///     extensions: None
/// }
/// ```
///
/// If parsing fails, `None` is returned.
pub fn parse(sdp: &[u8]) -> Option<types::IceCandidate> {
    match parsers::ice_candidate(sdp) {
        Done(i, o) => {
            if i.len() == 0 {
                Some(o)
            } else {
                None
            }
        },
        Incomplete(_) => None,
        Error(_) => None,
    }
}

#[cfg(test)]
mod tests {

    use std::net::{IpAddr, Ipv4Addr};

    use ::types::{Transport, CandidateType, IceCandidate};

    use super::parse;

    #[test]
    fn test_parse_full() {
        let candidate = b"candidate:842163049 1 udp 1686052607 1.2.3.4 46154 typ srflx raddr 10.0.0.17 rport 46154 generation 0 ufrag EEtu network-id 3 network-cost 10";
        let parsed: IceCandidate = parse(&candidate[..]).unwrap();
        println!("{:?}", parsed);
        assert_eq!(parsed.foundation, "842163049".to_string());
        assert_eq!(parsed.component_id, 1);
        assert_eq!(parsed.transport, Transport::Udp);
        assert_eq!(parsed.priority, 1686052607);
        assert_eq!(parsed.connection_address, IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)));
        assert_eq!(parsed.port, 46154);
        assert_eq!(parsed.candidate_type, CandidateType::Srflx);
        assert_eq!(parsed.rel_addr, Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 17))));
        assert_eq!(parsed.rel_port, Some(46154));
        assert!(parsed.extensions.is_some());
        let extensions = parsed.extensions.unwrap();
        assert_eq!(extensions.len(), 4);
        assert_eq!(extensions.get(&b"generation".to_vec()), Some(&b"0".to_vec()));
        assert_eq!(extensions.get(&b"ufrag".to_vec()), Some(&b"EEtu".to_vec()));
        assert_eq!(extensions.get(&b"network-id".to_vec()), Some(&b"3".to_vec()));
        assert_eq!(extensions.get(&b"network-cost".to_vec()), Some(&b"10".to_vec()));
    }

    #[test]
    fn test_parse_empty() {
        let parsed = parse(&b""[..]);
        assert_eq!(parsed, None);
    }

    #[test]
    fn test_parse_invalid() {
        let candidate = b"candidate:373990095 1 udp 41885439 asdf 5.148.189.205 63293 typ relay";
        let parsed = parse(&candidate[..]);
        assert_eq!(parsed, None);
    }

    #[test]
    fn test_parse_trailing_space() {
        let candidate = b"candidate:373990095 1 udp 41885439 5.148.189.205 63293 typ relay  ";
        let parsed = parse(&candidate[..]);
        assert_eq!(parsed, None);
    }

}
